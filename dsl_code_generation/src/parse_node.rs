use std::collections::HashMap;

use dsl_errors::{check, CodeGenError};
use linked_hash_map::LinkedHashMap;
use llvm_sys::core::{LLVMGetParam};

use dsl_lexer::ast::{
    FunctionDecleration, FunctionSignature, GenericParameters, ParseNode, TemplateDecleration,
    TypeSymbol, VariableDecleration,
};
use dsl_lexer::TokenKind;
use dsl_util::cast;

use crate::module::CodeGenPass;

use super::module::Module;
use dsl_symbol::{GenericType, Symbol, SymbolValue, Type, Value};

impl Module {
    pub(super) fn gen_parse_node(&self, node: &ParseNode) -> Value {
        match node {
            ParseNode::Expression(e, _) => {
                return self.gen_expression(e);
            }
            ParseNode::TemplateDecleration(TemplateDecleration {
                fields,
                generic,
                token,
                ..
            }) => {
                let name = cast!(&token.token_type, TokenKind::Ident);

                match &*self.code_gen_pass.borrow() {
                    CodeGenPass::Symbols => {
                        let template = check!(self, self.builder.create_struct_named(name), Value);
                        self.add_and_set_symbol(&name, SymbolValue::Template(template));

                        self.pop_symbol();
                    }
                    CodeGenPass::Values => {
                        let mut root_sym = self.symbol_root.borrow_mut();
                        let current =
                            self.get_symbol_mut(&mut root_sym, &self.current_symbol.borrow());
                        if let Some(Symbol { children, .. }) = current {
                            if let Some(Symbol {
                                value: SymbolValue::Template(ty),
                                children,
                                ..
                            }) = children.get_mut(name)
                            {
                                let mut vars = LinkedHashMap::new();

                                let types: Vec<_> = fields
                                    .iter()
                                    .map(|f| self.gen_type(&f.symbol_type))
                                    .collect();

                                for (f, ty) in fields.iter().zip(types.iter()) {
                                    let name = cast!(&f.symbol.token_type, TokenKind::Ident);
                                    vars.insert(name.clone(), ty.clone());

                                    children.insert(
                                        name.clone(),
                                        Symbol {
                                            name: name.clone(),
                                            value: SymbolValue::Field(ty.clone()),
                                            children: HashMap::new(),
                                        },
                                    );
                                }

                                self.builder.set_struct_body(ty, vars);
                            }
                        }
                    }
                    _ => (),
                }
            }
            ParseNode::VariableDecleration(VariableDecleration {
                identifier,
                possible_initializer,
                variable_type,
                ..
            }) => {
                match &*self.code_gen_pass.borrow() {
                    CodeGenPass::Values => (),
                    _ => return Value::Empty,
                }
                let place_var = if let Some((init, ..)) = possible_initializer {
                    let mut alloc = check!(
                        self,
                        self.builder.create_alloc(&self.builder.get_bool()),
                        Value
                    );

                    let value = check!(self.gen_expression(init.as_ref()));

                    if let Some(ty) = variable_type {
                        let ty = self.gen_type(ty.as_ref());

                        let nty = value.weak_cast(&ty, self.builder.get_builder());
                        let val = match nty {
                            Err(true) => {
                                self.add_error(
                                    "Initializer type does not match variable's declared type!"
                                        .to_string(),
                                );
                                return Value::Empty;
                            }
                            Err(false) => value,
                            Ok(v) => v,
                        };

                        check!(
                            self,
                            self.builder.set_allocated_type(
                                &mut alloc,
                                self.module,
                                &val,
                                val.get_type()
                            ),
                            Value
                        );

                        check!(
                            self,
                            self.builder.create_store(&alloc, &&val, self.module),
                            Value
                        );
                    } else {
                        check!(
                            self,
                            self.builder.set_allocated_type(
                                &mut alloc,
                                self.module,
                                &value,
                                value.get_type()
                            ),
                            Value
                        );

                        check!(
                            self,
                            self.builder.create_store(&alloc, &value, self.module),
                            Value
                        );
                    }

                    alloc
                } else {
                    if let Some(ty) = variable_type {
                        let ty = self.gen_type(ty.as_ref());
                        let place_var = check!(self, self.builder.create_alloc(&ty), Value);
                        place_var
                    } else {
                        self.add_error("Expected type or initializer!".to_string());
                        return Value::Empty;
                    }
                };

                let name = cast!(&identifier.token_type, TokenKind::Ident);
                self.get_current_mut(|f| {
                    if let Some(sym) = f {
                        sym.add_child(name, SymbolValue::Variable(place_var.clone()));
                    }
                })
            }
            ParseNode::FunctionDecleration(FunctionDecleration {
                body,
                function_type:
                    fty @ FunctionSignature {
                        parameters,
                        return_type,
                        ..
                    },
                identifier,
                generic,
                ..
            }) => {
                let name = cast!(&identifier.token_type, TokenKind::Ident);

                if let (Some(generic), CodeGenPass::Symbols) =
                    (generic, &*self.code_gen_pass.borrow())
                {
                    let mut path = self.current_symbol.borrow().clone();
                    path.push(name.clone());
                    let mut ty_params = LinkedHashMap::new();
                    match &**generic {
                        ParseNode::GenericParameters(GenericParameters { parameters, .. }) => {
                            let p: Result<(), ()> = parameters
                                .iter()
                                .map(|(tok, bounds, specs)| {
                                    if let Some(specs) = specs {
                                        Err(())
                                    } else {
                                        let str = cast!(&tok.token_type, TokenKind::Ident);
                                        let bounds = bounds.clone().map(|bnd| {
                                            bnd.iter().map(|t| self.gen_type(t)).collect()
                                        });
                                        ty_params.insert(str.clone(), GenericType::Generic(bounds));
                                        Ok(())
                                    }
                                })
                                .collect();
                            if p.is_err() {
                                return Value::Empty;
                            }
                        }
                        _ => (),
                    };

                    self.add_and_set_symbol(
                        &name,
                        SymbolValue::Funtion(Value::FunctionTemplate {
                            body: body.clone(),
                            ty: fty.clone(),
                            ty_params,
                            path,
                            existing: HashMap::new(),
                            specialization: HashMap::new(),
                        }),
                    );

                    {
                        let mut cur_sym = self.symbol_root.borrow_mut();
                        let current =
                            self.get_symbol_mut(&mut cur_sym, &self.current_symbol.borrow());

                        if let Some(c) = current {
                            match &**generic {
                                ParseNode::GenericParameters(GenericParameters {
                                    parameters,
                                    ..
                                }) => {
                                    parameters.iter().for_each(|(ident, bounds, specs)| {
                                        if let Some(specs) = specs {
                                            // let ty = self.gen_type(specs);
                                            // c.add_child(
                                            //     cast!(&ident.token_type, TokenKind::Ident),
                                            //     SymbolValue::Generic(
                                            //         Type::Empty,
                                            //         GenericType::Specialization(ty),
                                            //     ),
                                            // )
                                        } else {
                                            let bounds = if let Some(bounds) = bounds {
                                                let bounds: Vec<_> = bounds
                                                    .iter()
                                                    .map(|b| self.gen_type(b))
                                                    .collect();
                                                Some(bounds)
                                            } else {
                                                None
                                            };
                                            c.add_child(
                                                cast!(&ident.token_type, TokenKind::Ident),
                                                SymbolValue::Generic(
                                                    Type::Empty,
                                                    GenericType::Generic(bounds),
                                                ),
                                            )
                                        }
                                    });
                                }
                                _ => (),
                            };
                        }
                    }

                    self.pop_symbol();
                } else if let (Some(generic), CodeGenPass::SymbolsSpecialization) =
                    (generic, &*self.code_gen_pass.borrow())
                {
                    let mut path = self.current_symbol.borrow().clone();
                    path.push(name.clone());
                    let mut ty_params = LinkedHashMap::new();
                    match &**generic {
                        ParseNode::GenericParameters(GenericParameters { parameters, .. }) => {
                            let p = parameters.iter().fold(0, |acc, (tok, bounds, specs)| {
                                if let Some(specs) = specs {
                                    let str = cast!(&tok.token_type, TokenKind::Ident);
                                    let ty = self.gen_type(specs);
                                    ty_params.insert(str.clone(), GenericType::Specialization(ty));
                                    acc + 1
                                } else {
                                    let str = cast!(&tok.token_type, TokenKind::Ident);
                                    let bounds = bounds
                                        .clone()
                                        .map(|bnd| bnd.iter().map(|t| self.gen_type(t)).collect());
                                    ty_params.insert(str.clone(), GenericType::Generic(bounds));
                                    acc
                                }
                            });
                            if p == 0 {
                                return Value::Empty;
                            }
                        }
                        _ => (),
                    };

                    let name = self.get_next_name(&path[..path.len() - 1], name.to_string());

                    {
                        let mut cur_sym = self.symbol_root.borrow_mut();
                        let current = self.get_symbol_mut(&mut cur_sym, &path);

                        let mut npath = Vec::from(&path[..path.len() - 1]);
                        npath.push(name.clone());

                        let types = ty_params
                            .iter()
                            .map(|f| match f.1 {
                                GenericType::Generic(_) => "".to_string(),
                                GenericType::Specialization(s) => s.to_string(),
                            })
                            .collect();
                        if let Some(Symbol {
                            value:
                                SymbolValue::Funtion(Value::FunctionTemplate { specialization, .. }),
                            ..
                        }) = current
                        {
                            specialization.insert(types, npath);
                        }
                    }

                    self.add_and_set_symbol(
                        &name,
                        SymbolValue::Funtion(Value::FunctionTemplate {
                            body: body.clone(),
                            ty: fty.clone(),
                            ty_params,
                            path,
                            existing: HashMap::new(),
                            specialization: HashMap::new(),
                        }),
                    );

                    {
                        let mut cur_sym = self.symbol_root.borrow_mut();
                        let current =
                            self.get_symbol_mut(&mut cur_sym, &self.current_symbol.borrow());

                        if let Some(c) = current {
                            match &**generic {
                                ParseNode::GenericParameters(GenericParameters {
                                    parameters,
                                    ..
                                }) => {
                                    parameters.iter().for_each(|(ident, bounds, specs)| {
                                        if let Some(specs) = specs {
                                            let ty = self.gen_type(specs);
                                            c.add_child(
                                                cast!(&ident.token_type, TokenKind::Ident),
                                                SymbolValue::Generic(
                                                    Type::Empty,
                                                    GenericType::Specialization(ty),
                                                ),
                                            )
                                        } else {
                                        }
                                    });
                                }
                                _ => (),
                            };
                        }
                    }

                    self.pop_symbol();
                } else {
                    if let CodeGenPass::Symbols = *self.code_gen_pass.borrow() {
                        let return_type = self.gen_type(return_type);

                        let types: Vec<(String, Type)> = parameters
                            .iter()
                            .map(|f| {
                                (
                                    cast!(&f.symbol.token_type, TokenKind::Ident).clone(),
                                    self.gen_type(&f.symbol_type),
                                )
                            })
                            .collect();

                        let function_type = self.builder.get_fn(return_type.clone(), &types);

                        let function = check!(
                            self,
                            self.builder
                                .add_function(function_type, name.to_string(), self.module),
                            Value
                        );

                        self.add_and_set_symbol(&name, SymbolValue::Funtion(function));

                        {
                            let mut cur_sym = self.symbol_root.borrow_mut();
                            let current =
                                self.get_symbol_mut(&mut cur_sym, &self.current_symbol.borrow());

                            if let Some(c) = current {
                                for (name, ty) in types.iter() {
                                    c.add_child(
                                        &name,
                                        SymbolValue::Variable(Value::Variable {
                                            llvm_value: std::ptr::null_mut(),
                                            variable_type: ty.clone(),
                                        }),
                                    );
                                }
                            }
                        }

                        self.pop_symbol();
                    } else if let CodeGenPass::Values = &*self.code_gen_pass.borrow() {
                        self.current_symbol.borrow_mut().push(name.clone());

                        let potato = {
                            let sym = self.symbol_root.borrow();
                            let current = self.get_symbol(&sym, &self.current_symbol.borrow());

                            if let Some(Symbol {
                                value:
                                    SymbolValue::Funtion(
                                        function @ Value::Function {
                                            function_type:
                                                Type::Function {
                                                    return_type,
                                                    parameters,
                                                    ..
                                                },
                                            ..
                                        },
                                    ),
                                ..
                            }) = current
                            {
                                let block =
                                    check!(self, self.builder.append_block(&function), Value);

                                self.current_block.replace(block);
                                self.builder.set_position_end(&self.current_block.borrow());

                                self.current_function.replace(function.clone());

                                let pallocs: Result<Vec<Value>, _> = parameters
                                    .iter()
                                    .map(|(_name, ty)| self.builder.create_alloc(ty))
                                    .collect();
                                let pallocs = check!(self, pallocs, Value);

                                let res: Result<(), _> = pallocs
                                    .iter()
                                    .enumerate()
                                    .map(|(i, alloc)| {
                                        let param = unsafe {
                                            if let Ok(p) = self
                                                .current_function
                                                .borrow()
                                                .get_value(self.builder.get_builder(), self.module)
                                            {
                                                LLVMGetParam(p, i.try_into().unwrap())
                                            } else {
                                                return Err(CodeGenError {
                                                    message: "Unable to get function value!"
                                                        .to_owned(),
                                                });
                                            }
                                        };
                                        self.builder.create_store_raw_val(alloc, param)?;
                                        Ok(())
                                    })
                                    .collect();

                                check!(self, res, Value);

                                let alloc = match &**return_type {
                                    Type::Unit { .. } => None,
                                    ty => Some(check!(self, self.builder.create_alloc(ty), Value)),
                                };

                                Some((alloc, pallocs))
                            } else {
                                None
                            }
                        };

                        if let Some((alloc, pallocs)) = potato {
                            {
                                let mut cur_sym = self.symbol_root.borrow_mut();
                                let current = self
                                    .get_symbol_mut(&mut cur_sym, &self.current_symbol.borrow());

                                if let Some(c) = current {
                                    for (TypeSymbol { symbol, .. }, alloc) in
                                        parameters.iter().zip(pallocs.into_iter())
                                    {
                                        let name = cast!(&symbol.token_type, TokenKind::Ident);
                                        if let Some(Symbol {
                                            value: SymbolValue::Variable(value),
                                            ..
                                        }) = c.children.get_mut(name)
                                        {
                                            *value = alloc
                                        }
                                    }
                                }
                            }

                            let val = self.gen_parse_node(body.as_ref());

                            if let Some(alloc) = alloc {
                                if val.has_value() {
                                    check!(
                                        self,
                                        self.builder.create_store(&alloc, &val, self.module),
                                        Value
                                    );
                                }

                                check!(self, self.builder.create_ret(&alloc), Value);
                            } else {
                                self.builder.create_ret_void();
                            }
                        }

                        self.pop_symbol();
                    }
                }
            }

            _ => (),
        };
        Value::Empty
    }
}
