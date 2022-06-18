use dsl_errors::{check, CodeGenError};
use llvm_sys::core::LLVMGetParam;

use dsl_lexer::ast::{FunctionDecleration, FunctionSignature, ParseNode, VariableDecleration};
use dsl_lexer::TokenKind;
use dsl_util::cast;

use crate::module::CodeGenPass;

use super::module::Module;
use dsl_symbol::{Symbol, SymbolValue, Type, Value};

impl Module {
    pub(super) fn gen_parse_node(&self, node: &ParseNode) -> Value {
        match node {
            ParseNode::Expression(e, _) => {
                return self.gen_expression(e);
            }
            ParseNode::VariableDecleration(VariableDecleration {
                identifier,
                possible_initializer,
                variable_type,
                ..
            }) => {
                let place_var = if let Some((init, ..)) = possible_initializer {
                    let alloc_block = check!(
                        self,
                        self.builder.append_block(&self.current_function.borrow()),
                        Value
                    );
                    let expr = check!(
                        self,
                        self.builder.append_block(&self.current_function.borrow()),
                        Value
                    );

                    let end = check!(self, self.builder.create_block(), Value);

                    check!(self, self.builder.create_branch(&alloc_block), Value);

                    self.builder.set_position_end(&expr);

                    self.current_block.replace(expr);
                    let value = check!(self.gen_expression(init.as_ref()));
                    check!(self, self.builder.create_branch(&end), Value);

                    self.builder.set_position_end(&alloc_block);

                    if let Some(ty) = variable_type {
                        let ty = self.gen_type(ty.as_ref());
                        let var = check!(self, self.builder.create_alloc(&ty), Value);

                        check!(
                            self,
                            self.builder.create_branch(&self.current_block.borrow()),
                            Value
                        );

                        check!(
                            self,
                            self.builder
                                .append_existing_block(&self.current_function.borrow(), &end),
                            Value
                        );

                        self.builder.set_position_end(&end);

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

                        check!(self, self.builder.create_store(&var, &&val), Value);

                        var
                    } else {
                        let var = check!(self, self.builder.create_alloc(&value.get_type()), Value);
                        check!(
                            self,
                            self.builder.create_branch(&self.current_block.borrow()),
                            Value
                        );

                        check!(
                            self,
                            self.builder
                                .append_existing_block(&self.current_function.borrow(), &end),
                            Value
                        );

                        self.builder.set_position_end(&end);

                        check!(self, self.builder.create_store(&var, &value), Value);

                        var
                    }
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
                    FunctionSignature {
                        parameters,
                        return_type,
                        ..
                    },
                identifier,
                generic,
                ..
            }) => {
                let name = cast!(&identifier.token_type, TokenKind::Ident);

                if let Some(generic) = generic {
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
                    } else {
                        self.current_symbol.borrow_mut().push(name.clone());

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
                            let block = check!(self, self.builder.append_block(&function), Value);

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
                                            .get_value(self.builder.get_builder())
                                        {
                                            LLVMGetParam(p, i.try_into().unwrap())
                                        } else {
                                            return Err(CodeGenError {
                                                message: "Unable to get function value!".to_owned(),
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

                            let val = self.gen_parse_node(body.as_ref());

                            if let Some(alloc) = alloc {
                                if val.has_value() {
                                    check!(self, self.builder.create_store(&alloc, &val), Value);
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
