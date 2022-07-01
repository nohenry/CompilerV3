use std::collections::HashMap;

use dsl_errors::{check, CodeGenError};
use dsl_llvm::IRBuilder;
use linked_hash_map::LinkedHashMap;
use llvm_sys::core::LLVMGetParam;

use crate::module::CodeGenPass;
use dsl_lexer::{
    ast::{
        ActionDecleration, Expression, FunctionDecleration, FunctionSignature, GenericParameters,
        ImportDecleration, ParseNode, TemplateDecleration, TypeSymbol, VariableDecleration,
    },
    Keyword, Token, TokenKind,
};

use super::module::Module;
use dsl_symbol::{GenericType, Symbol, SymbolFlags, SymbolValue, Type, Value};

impl Module {
    pub(super) fn gen_parse_node(&self, node: &ParseNode) -> Value {
        match node {
            ParseNode::Expression(e, _) => {
                return self.gen_expression(e);
            }
            ParseNode::Export(b, _) => {
                self.flags_to_apply
                    .borrow_mut()
                    .set(SymbolFlags::EXPORT, true);
                return self.gen_parse_node(&b);
            }
            ParseNode::Import(ImportDecleration { path, .. }) => {
                let pt: Option<Vec<String>> = path
                    .iter()
                    .map(|p| {
                        if let Expression::Identifier(t) = p {
                            Some(t.as_string())
                        } else {
                            None
                        }
                    })
                    .collect();

                if let Some(pt) = pt {
                    self.imports.borrow_mut().push(pt);
                } else {
                    self.add_error("Invalid expression for import decleration!".into());
                }
            }
            ParseNode::TemplateDecleration(TemplateDecleration {
                fields,
                generic,
                token,
                ..
            }) => {
                let name = token.as_string();

                if let (Some(generic), CodeGenPass::TemplateSymbols) =
                    (generic, &*self.code_gen_pass.borrow())
                {
                    let mut path = self.current_symbol.borrow().clone();
                    path.push(name.clone());

                    let ty_params = if let Ok(ty) = self.gen_generic(generic) {
                        ty
                    } else {
                        return Value::Empty;
                    };

                    self.add_and_set_symbol(
                        &name,
                        SymbolValue::Template(Type::TemplateTemplate {
                            fields: fields.clone(),
                            path,
                            ty_params,
                            existing: HashMap::new(),
                            specialization: HashMap::new(),
                        }),
                    );

                    check!(
                        self.errors.borrow_mut(),
                        self.add_generic_children(generic),
                        Value
                    );

                    self.pop_symbol();
                } else if let (Some(generic), CodeGenPass::TemplateSpecialization) =
                    (generic, &*self.code_gen_pass.borrow())
                {
                    let mut path = self.current_symbol.borrow().clone();
                    path.push(name.clone());

                    let ty_params = if let Ok(ty) = self.gen_special_generic(generic) {
                        ty
                    } else {
                        return Value::Empty;
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
                                SymbolValue::Template(Type::TemplateTemplate { specialization, .. }),
                            ..
                        }) = current
                        {
                            specialization.insert(types, npath);
                        }
                    }

                    self.add_and_set_symbol(
                        &name,
                        SymbolValue::Template(Type::TemplateTemplate {
                            fields: fields.clone(),
                            path,
                            ty_params,
                            existing: HashMap::new(),
                            specialization: HashMap::new(),
                        }),
                    );

                    check!(
                        self.errors.borrow_mut(),
                        self.add_special_generic_children(generic),
                        Value
                    );

                    self.pop_symbol();
                } else if let (Some(generic), CodeGenPass::TemplateValues) =
                    (generic, &*self.code_gen_pass.borrow())
                {
                    let mut path = self.current_symbol.borrow().clone();
                    path.push(name.clone());

                    let path = {
                        let mut root_sym = self.symbol_root.borrow_mut();
                        let current = self.get_symbol_mut(&mut root_sym, &path);

                        if let Ok(ty) = self.gen_special_generic(generic) {
                            if let Some(Symbol {
                                value:
                                    SymbolValue::Template(Type::TemplateTemplate {
                                        specialization, ..
                                    }),
                                ..
                            }) = current
                            {
                                let st: Vec<String> = ty.values().map(|v| v.to_string()).collect();
                                let specialization = specialization.iter().find(|(f, _)| {
                                    for (a, b) in f.iter().zip(st.iter()) {
                                        if a == "" {
                                            return true;
                                        } else if a != b {
                                            return false;
                                        } else {
                                            return true;
                                        }
                                    }
                                    true
                                });

                                if let Some((_, path)) = specialization {
                                    path.clone()
                                } else {
                                    path
                                }
                            } else {
                                path
                            }
                        } else {
                            path
                        }
                    };

                    let old_sym = self.current_symbol.replace(path);

                    let types: Vec<_> = fields
                        .iter()
                        .map(|f| (f.symbol.as_string(), self.gen_type(&f.symbol_type)))
                        .collect();

                    {
                        let mut root_sym = self.symbol_root.borrow_mut();
                        let current =
                            self.get_symbol_mut(&mut root_sym, &self.current_symbol.borrow());

                        if let Some(Symbol {
                            value: SymbolValue::Template(Type::TemplateTemplate { .. }),
                            children,
                            ..
                        }) = current
                        {
                            for (name, ty) in types.into_iter() {
                                children.insert(
                                    name.clone(),
                                    Symbol::new(name.clone(), SymbolValue::Field(ty)),
                                );
                            }
                        }
                    }

                    self.current_symbol.replace(old_sym);
                } else {
                    match &*self.code_gen_pass.borrow() {
                        CodeGenPass::TemplateSymbols => {
                            let mng = self.get_mangled_name(&name);
                            let mut pth = self.current_symbol.borrow().clone();
                            pth.push(name.clone());
                            let template = check!(
                                self.errors.borrow_mut(),
                                self.builder.create_struct_named(&pth, None, &mng),
                                Value
                            );
                            self.add_and_set_symbol(&name, SymbolValue::Template(template));

                            self.pop_symbol();
                        }
                        CodeGenPass::TemplateValues => {
                            let types: Vec<_> = fields
                                .iter()
                                .map(|f| self.gen_type(&f.symbol_type))
                                .collect();

                            let mut root_sym = self.symbol_root.borrow_mut();
                            let current =
                                self.get_symbol_mut(&mut root_sym, &self.current_symbol.borrow());
                            if let Some(Symbol { children, .. }) = current {
                                if let Some(Symbol {
                                    value: SymbolValue::Template(ty),
                                    children,
                                    ..
                                }) = children.get_mut(&name)
                                {
                                    let mut vars = LinkedHashMap::new();

                                    for (f, ty) in fields.iter().zip(types.iter()) {
                                        let name = f.symbol.as_string();
                                        vars.insert(name.clone(), ty.clone());

                                        children.insert(
                                            name.clone(),
                                            Symbol::new(
                                                name.clone(),
                                                SymbolValue::Field(ty.clone()),
                                            ),
                                        );
                                    }

                                    self.builder.set_struct_body(ty, vars);
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
            ParseNode::ActionDecleration(ActionDecleration {
                template_type,
                body,
                generic,
                ..
            }) => {
                if let Some(generic) = generic {
                    let dsl_lexer::ast::Type::GenericType(dsl_lexer::ast::GenericType {base_type: box dsl_lexer::ast::Type::NamedType(tok), ..}) = template_type else {
                        // TODO: errors
                        return Value::Empty;
                    };
                    let ParseNode::GenericParameters(ps1) = &**generic else {
                        // TODO: errors
                        return Value::Empty;
                    };

                    let name = tok.as_string();
                    let sym = &mut *self.symbol_root.borrow_mut();
                    let ty = self.find_up_chain_mut(sym, &self.current_symbol.borrow(), &name);
                    let Some(fnd) = ty else {
                        // TODO: errors
                        return Value::Empty;
                    };

                    if let ParseNode::Expression(Expression::Block(st, _), _) = &**body {
                        for fun in st {
                            if let ParseNode::FunctionDecleration(func) = fun {
                                let ngen = if let Some(fngen) = &func.generic {
                                    let ParseNode::GenericParameters(ps2) = &**fngen  else {
                                        // TODO: errors
                                        return Value::Empty;
                                    };
                                    if !ps1
                                        .parameters
                                        .iter()
                                        .map(|f| &f.0)
                                        .ne(ps1.parameters.iter().map(|f| &f.0))
                                    {
                                        self.add_error("Generic parameter redefined!".into());
                                        return Value::Empty;
                                    }

                                    ps1.parameters
                                        .to_vec()
                                        .into_iter()
                                        .chain(ps2.parameters.to_vec().into_iter())
                                        .collect()
                                } else {
                                    ps1.parameters.to_vec()
                                };

                                let fngen = ParseNode::GenericParameters(GenericParameters {
                                    parameters: ngen.clone(),
                                    range: ps1.range,
                                });
                                let fngen = check!(
                                    self.errors.borrow_mut(),
                                    self.gen_generic(&fngen),
                                    Value
                                );

                                let mut path = self.current_symbol.borrow().clone();
                                path.push(name.clone());
                                let name = func.identifier.as_string();
                                path.push(name.clone());

                                let mut i = func.function_type.parameters.clone().into_iter();
                                let prms: Vec<TypeSymbol> = match i.next() {
                                    Some(f) => {
                                        if let dsl_lexer::ast::Type::ConstSelf = &f.symbol_type {
                                            [TypeSymbol {
                                                symbol: Token {
                                                    token_type: TokenKind::Keyword(Keyword {
                                                        keyword: dsl_lexer::KeywordKind::Const,
                                                        range: f.symbol.range,
                                                    }),
                                                    range: f.symbol.range,
                                                },
                                                symbol_type: template_type.clone(),
                                            }]
                                            .into_iter()
                                            .chain(i)
                                            .collect()
                                        } else if let dsl_lexer::ast::Type::SELF = &f.symbol_type {
                                            [TypeSymbol {
                                                symbol: f.symbol.clone(),
                                                symbol_type: template_type.clone(),
                                            }]
                                            .into_iter()
                                            .chain(i)
                                            .collect()
                                        } else {
                                            i.collect()
                                        }
                                    }
                                    None => i.collect(),
                                };

                                let sym = fnd.add_child(
                                    &name,
                                    SymbolValue::Funtion(Value::FunctionTemplate {
                                        body: func.body.clone(),
                                        existing: Default::default(),
                                        path: path,
                                        specialization: Default::default(),
                                        ty: FunctionSignature {
                                            parameters: prms,
                                            ..func.function_type.clone()
                                        },
                                        ty_params: fngen,
                                    }),
                                );

                                ngen.iter().for_each(|(ident, bounds, specs)| {
                                    if let Some(_) = specs {
                                    } else {
                                        let bounds = if let Some(bounds) = bounds {
                                            let bounds: Vec<_> =
                                                bounds.iter().map(|b| self.gen_type(b)).collect();
                                            Some(bounds)
                                        } else {
                                            None
                                        };
                                        sym.add_child(
                                            &ident.as_string(),
                                            SymbolValue::Generic(
                                                Type::Empty,
                                                GenericType::Generic(bounds),
                                            ),
                                        );
                                    }
                                });
                            }
                        }
                    }
                } else {
                    let temp = self.gen_type(template_type);
                    let Some(path)= temp.resolve_path() else {
                        // TODO: errors
                        return Value::Empty;
                    };

                    if let Type::TemplateTemplate { path, .. } = &temp {
                        if let Some(s) = path.last() {
                            if let CodeGenPass::Symbols = &*self.code_gen_pass.borrow() {
                                self.add_error(format!(
                                    "Template type `{}` requires a generic argument",
                                    s
                                ));
                            }
                        }
                        return Value::Empty;
                    }

                    let old_current_sym = self.current_symbol.take();

                    self.current_symbol.replace(path);

                    self.gen_parse_node(body);

                    self.current_symbol.replace(old_current_sym);
                }
            }
            ParseNode::VariableDecleration(VariableDecleration {
                identifier,
                possible_initializer,
                variable_type,
                is_const,
                ..
            }) => {
                match &*self.code_gen_pass.borrow() {
                    CodeGenPass::Values => (),
                    _ => return Value::Empty,
                }
                let place_var = if let Some((init, ..)) = possible_initializer {
                    let mut alloc = check!(
                        self.errors.borrow_mut(),
                        self.builder.create_alloc(&IRBuilder::get_bool(), *is_const),
                        Value
                    );

                    let value = check!(self.gen_expression(init.as_ref()), Value);

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
                            self.errors.borrow_mut(),
                            self.builder.set_allocated_type(
                                &mut alloc,
                                self.module,
                                val.get_type()
                            ),
                            Value
                        );

                        check!(
                            self.errors.borrow_mut(),
                            self.builder.create_store(&alloc, &&val, self.module),
                            Value
                        );
                    } else {
                        check!(
                            self.errors.borrow_mut(),
                            self.builder.set_allocated_type(
                                &mut alloc,
                                self.module,
                                value.get_type()
                            ),
                            Value
                        );

                        check!(
                            self.errors.borrow_mut(),
                            self.builder.create_store(&alloc, &value, self.module),
                            Value
                        );
                    }

                    alloc
                } else {
                    if let Some(ty) = variable_type {
                        let ty = self.gen_type(ty.as_ref());
                        let place_var = check!(
                            self.errors.borrow_mut(),
                            self.builder.create_alloc(&ty, *is_const),
                            Value
                        );
                        place_var
                    } else {
                        self.add_error("Expected type or initializer!".to_string());
                        return Value::Empty;
                    }
                };

                let name = identifier.as_string();
                self.get_current_mut(|f| {
                    if let Some(sym) = f {
                        if *is_const {
                            sym.add_child_flags(
                                &name,
                                SymbolValue::Variable(place_var.clone()),
                                SymbolFlags::CONSTANT,
                            );
                        } else {
                            sym.add_child(&name, SymbolValue::Variable(place_var.clone()));
                        }
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
                let name = identifier.as_string();

                if let (Some(generic), CodeGenPass::Symbols) =
                    (generic, &*self.code_gen_pass.borrow())
                {
                    let mut path = self.current_symbol.borrow().clone();
                    path.push(name.clone());

                    let ty_params = if let Ok(ty) = self.gen_generic(generic) {
                        ty
                    } else {
                        return Value::Empty;
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

                    check!(
                        self.errors.borrow_mut(),
                        self.add_generic_children(generic),
                        Value
                    );

                    self.pop_symbol();
                } else if let (Some(generic), CodeGenPass::SymbolsSpecialization) =
                    (generic, &*self.code_gen_pass.borrow())
                {
                    let mut path = self.current_symbol.borrow().clone();
                    path.push(name.clone());

                    let ty_params = if let Ok(ty) = self.gen_special_generic(generic) {
                        ty
                    } else {
                        return Value::Empty;
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

                    check!(
                        self.errors.borrow_mut(),
                        self.add_special_generic_children(generic),
                        Value
                    );

                    self.pop_symbol();
                } else {
                    if let CodeGenPass::Symbols = *self.code_gen_pass.borrow() {
                        let return_type = self.gen_type(return_type);

                        let types: Option<Vec<(String, Type)>> = parameters
                            .iter()
                            .map(|f| {
                                if &f.symbol.as_string() == "self"
                                    || &f.symbol.as_string() == "const self"
                                {
                                    let sym = self.symbol_root.borrow();
                                    let current =
                                        self.get_symbol(&sym, &self.current_symbol.borrow());

                                    if let Some(Symbol {
                                        value: SymbolValue::Template(t) | SymbolValue::Primitive(t),
                                        ..
                                    }) = current
                                    {
                                        if let dsl_lexer::ast::Type::ConstSelf = f.symbol_type {
                                            Some((
                                                f.symbol.as_string().clone(),
                                                t.clone().get_ptr(true),
                                            ))
                                        } else if let dsl_lexer::ast::Type::SELF = f.symbol_type {
                                            Some((
                                                f.symbol.as_string().clone(),
                                                t.clone().get_ptr(false),
                                            ))
                                        } else {
                                            panic!("Typejsconst mismathc idk (1)")
                                        }
                                    } else {
                                        None
                                    }
                                } else {
                                    Some((
                                        f.symbol.as_string().clone(),
                                        self.gen_type(&f.symbol_type),
                                    ))
                                }
                            })
                            .collect();

                        let Some(types) = types else {
                            //TODO: add error 
                            return Value::Empty
                        };

                        let function_type = IRBuilder::get_fn(return_type.clone(), &types);

                        let function = check!(
                            self.errors.borrow_mut(),
                            self.builder.add_function(
                                function_type,
                                self.get_mangled_name(&name),
                                self.module
                            ),
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
                                            constant: true,
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
                                let block = check!(
                                    self.errors.borrow_mut(),
                                    self.builder.append_block(&function),
                                    Value
                                );

                                self.current_block.replace(block);
                                self.builder.set_position_end(&self.current_block.borrow());

                                self.current_function.replace(function.clone());

                                let pallocs: Result<Vec<Value>, _> = parameters
                                    .iter()
                                    .map(|(_name, ty)| self.builder.create_alloc(ty, true))
                                    .collect();
                                let pallocs = check!(self.errors.borrow_mut(), pallocs, Value);

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

                                check!(self.errors.borrow_mut(), res, Value);

                                let alloc = match &**return_type {
                                    Type::Unit { .. } => None,
                                    ty => Some(check!(
                                        self.errors.borrow_mut(),
                                        self.builder.create_alloc(ty, false),
                                        Value
                                    )),
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
                                        let name = symbol.as_string();
                                        if let Some(Symbol {
                                            value: SymbolValue::Variable(value),
                                            ..
                                        }) = c.children.get_mut(&name)
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
                                        self.errors.borrow_mut(),
                                        self.builder.create_store(&alloc, &val, self.module),
                                        Value
                                    );
                                }

                                check!(
                                    self.errors.borrow_mut(),
                                    self.builder.create_ret(&alloc),
                                    Value
                                );
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

    pub fn gen_generic(
        &self,
        node: &ParseNode,
    ) -> Result<LinkedHashMap<String, GenericType>, CodeGenError> {
        match node {
            ParseNode::GenericParameters(GenericParameters { parameters, .. }) => {
                let mut ty_params = LinkedHashMap::new();
                let p: Result<(), ()> = parameters
                    .iter()
                    .map(|(tok, bounds, specs)| {
                        if let Some(specs) = specs {
                            Err(())
                        } else {
                            let str = tok.as_string();
                            let bounds = bounds
                                .clone()
                                .map(|bnd| bnd.iter().map(|t| self.gen_type(t)).collect());
                            ty_params.insert(str.clone(), GenericType::Generic(bounds));
                            Ok(())
                        }
                    })
                    .collect();
                if p.is_err() {
                    return Err(CodeGenError {
                        message: "Unable to parse generic parameters (1)".into(),
                    });
                }
                Ok(ty_params)
            }
            _ => Err(CodeGenError {
                message: "Unable to parse generic parameters (2)".into(),
            }),
        }
    }

    pub fn gen_special_generic(
        &self,
        node: &ParseNode,
    ) -> Result<LinkedHashMap<String, GenericType>, CodeGenError> {
        match node {
            ParseNode::GenericParameters(GenericParameters { parameters, .. }) => {
                let mut ty_params = LinkedHashMap::new();
                let p = parameters.iter().fold(0, |acc, (tok, bounds, specs)| {
                    if let Some(specs) = specs {
                        let str = tok.as_string();
                        let ty = self.gen_type(specs);
                        ty_params.insert(str.clone(), GenericType::Specialization(ty));
                        acc + 1
                    } else {
                        let str = tok.as_string();
                        let bounds = bounds
                            .clone()
                            .map(|bnd| bnd.iter().map(|t| self.gen_type(t)).collect());
                        ty_params.insert(str.clone(), GenericType::Generic(bounds));
                        acc
                    }
                });
                if p == 0 {
                    return Err(CodeGenError {
                        message: "Unable to parse generic parameters (3)".into(),
                    });
                }
                Ok(ty_params)
            }
            _ => Err(CodeGenError {
                message: "Unable to parse generic parameters (4)".into(),
            }),
        }
    }

    pub fn add_generic_children(&self, node: &ParseNode) -> Result<(), CodeGenError> {
        match node {
            ParseNode::GenericParameters(GenericParameters { parameters, .. }) => {
                parameters.iter().for_each(|(ident, bounds, specs)| {
                    if let Some(specs) = specs {
                    } else {
                        let bounds = if let Some(bounds) = bounds {
                            let bounds: Vec<_> = bounds.iter().map(|b| self.gen_type(b)).collect();
                            Some(bounds)
                        } else {
                            None
                        };
                        self.add_symbol(
                            &ident.as_string(),
                            SymbolValue::Generic(Type::Empty, GenericType::Generic(bounds)),
                        )
                    }
                });
                Ok(())
            }
            _ => Err(CodeGenError {
                message: "Unable to add generic parameter to symbol tree!".into(),
            }),
        }
    }

    pub fn add_special_generic_children(&self, node: &ParseNode) -> Result<(), CodeGenError> {
        match node {
            ParseNode::GenericParameters(GenericParameters { parameters, .. }) => {
                parameters.iter().for_each(|(ident, bounds, specs)| {
                    if let Some(specs) = specs {
                        let ty = self.gen_type(specs);
                        self.add_symbol(
                            &ident.as_string(),
                            SymbolValue::Generic(Type::Empty, GenericType::Specialization(ty)),
                        )
                    } else {
                    }
                });
                Ok(())
            }
            _ => Err(CodeGenError {
                message: "Unable to add generic parameter to symbol tree!".into(),
            }),
        }
    }
}
