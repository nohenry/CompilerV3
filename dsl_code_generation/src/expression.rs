use dsl_errors::{check, CodeGenError};
use llvm_sys::core::LLVMGetParam;
use llvm_sys::{
    LLVMGetInsertPoint, LLVMIntPredicate, LLVMOpcode, LLVMRealPredicate, LLVMSetInsertPoint,
};

use dsl_lexer::ast::{
    BinaryExpression, Expression, FunctionCall, IfExpression, IndexExpression, Loop,
    LoopExpression, UnaryExpression,
};
use dsl_lexer::{OperatorKind, Token, TokenKind};
use dsl_util::cast;

use super::module::Module;
use dsl_symbol::{Symbol, SymbolValue, Type, Value};

impl Module {
    pub(super) fn gen_expression(&self, expression: &Expression) -> Value {
        match expression {
            Expression::BinaryExpression(BinaryExpression {
                left,
                operator,
                right,
                ..
            }) => {
                use dsl_lexer::OperatorKind::*;
                let func = match operator {
                    Assignment => {
                        let left = self.gen_expression(left);
                        let right = self.gen_expression(right);

                        check!(self, self.builder.create_store(&left, &right), Value);

                        return Value::Empty;
                    }
                    Plus => LLVMOpcode::LLVMAdd,
                    Minus => LLVMOpcode::LLVMSub,
                    Mult => LLVMOpcode::LLVMMul,
                    Divide => LLVMOpcode::LLVMSDiv,
                    BitAnd => LLVMOpcode::LLVMAnd,
                    BitOr => LLVMOpcode::LLVMOr,
                    BitXor => LLVMOpcode::LLVMXor,
                    Percent => LLVMOpcode::LLVMSRem,
                    BitLeft => LLVMOpcode::LLVMShl,
                    BitRight => LLVMOpcode::LLVMAShr,
                    c => {
                        // Handle operator equal
                        let oper = match c {
                            PlusEqual => LLVMOpcode::LLVMAdd,
                            MinusEqual => LLVMOpcode::LLVMSub,
                            MultEqual => LLVMOpcode::LLVMMul,
                            DivideEqual => LLVMOpcode::LLVMSDiv,
                            BitAndEqual => LLVMOpcode::LLVMAnd,
                            BitOrEqual => LLVMOpcode::LLVMOr,
                            BitXorEqual => LLVMOpcode::LLVMXor,
                            PercentEqual => LLVMOpcode::LLVMSRem,
                            BitLeftEqual => LLVMOpcode::LLVMShl,
                            BitRightEqual => LLVMOpcode::LLVMAShr,
                            _ => {
                                let left = self.gen_expression(left);
                                let right = self.gen_expression(right);
                                match left.get_type() {
                                    Type::Integer { signed, .. } => {
                                        let func = match c {
                                            Eq => LLVMIntPredicate::LLVMIntEQ,
                                            NEq => LLVMIntPredicate::LLVMIntNE,
                                            Lt if *signed => LLVMIntPredicate::LLVMIntSLT,
                                            LtEq if *signed => LLVMIntPredicate::LLVMIntSLE,
                                            Gt if *signed => LLVMIntPredicate::LLVMIntSGT,
                                            GtEq if *signed => LLVMIntPredicate::LLVMIntSGE,
                                            NGt if *signed => LLVMIntPredicate::LLVMIntSLE,
                                            NLt if *signed => LLVMIntPredicate::LLVMIntSGE,

                                            Lt if !*signed => LLVMIntPredicate::LLVMIntULT,
                                            LtEq if !*signed => LLVMIntPredicate::LLVMIntULE,
                                            Gt if !*signed => LLVMIntPredicate::LLVMIntUGT,
                                            GtEq if !*signed => LLVMIntPredicate::LLVMIntUGE,
                                            NGt if !*signed => LLVMIntPredicate::LLVMIntULE,
                                            NLt if !*signed => LLVMIntPredicate::LLVMIntUGE,
                                            _ => {
                                                self.add_error(format!(
                                                    "Unsupported binary comparison operation {:?}",
                                                    c
                                                ));
                                                return Value::Empty;
                                            }
                                        };
                                        return check!(
                                            self,
                                            self.builder.create_icompare(&left, &right, func),
                                            Value
                                        );
                                    }
                                    Type::Boolean { .. } => {
                                        let func = match c {
                                            Eq => LLVMIntPredicate::LLVMIntEQ,
                                            NEq => LLVMIntPredicate::LLVMIntNE,
                                            _ => {
                                                self.add_error(format!(
                                                    "Unsupported binary comparison operation {:?}",
                                                    c
                                                ));
                                                return Value::Empty;
                                            }
                                        };
                                        return check!(
                                            self,
                                            self.builder.create_icompare(&left, &right, func),
                                            Value
                                        );
                                    }
                                    Type::Float { .. } => {
                                        let func = match c {
                                            Eq => LLVMRealPredicate::LLVMRealOEQ,
                                            _ => {
                                                self.add_error(format!(
                                                    "Unsupported binary operation {:?}",
                                                    c
                                                ));
                                                return Value::Empty;
                                            }
                                        };
                                        return check!(
                                            self,
                                            self.builder.create_fcompare(&left, &right, func),
                                            Value
                                        );
                                    }
                                    _ => {
                                        self.add_error(format!(
                                            "Unsupported binary operation {:?}",
                                            c
                                        ));
                                        return Value::Empty;
                                    }
                                }
                            }
                        };

                        let left = self.gen_expression(left);
                        let right = self.gen_expression(right);

                        let op =
                            check!(self, self.builder.create_bin_op(&left, &right, oper), Value);
                        check!(self, self.builder.create_store(&left, &op), Value);

                        // load modify and store value for op=
                        return Value::Empty;
                    }
                };

                let left = self.gen_expression(left);
                let right = self.gen_expression(right);

                check!(self, self.builder.create_bin_op(&left, &right, func), Value)
            }
            Expression::UnaryExpression(UnaryExpression {
                expression,
                operator,
                ..
            }) => match operator {
                OperatorKind::Minus => {
                    let expr = self.gen_expression(&expression);

                    check!(self, self.builder.create_neg(&expr), Value)
                }
                OperatorKind::BitAnd => {
                    let value = self.gen_expression(&expression);

                    match value {
                        Value::Variable {
                            llvm_value,
                            variable_type,
                        } => Value::Literal {
                            llvm_value,
                            literal_type: self.builder.get_ptr(&variable_type),
                        },
                        _ => Value::Empty,
                    }
                }
                OperatorKind::Mult => {
                    let value = self.gen_expression(&expression);
                    match &value {
                        Value::Variable { .. } => {
                            let value = check!(self, self.builder.create_load(&value), Value);

                            match value {
                                Value::Literal {
                                    llvm_value,
                                    literal_type: Type::Reference { base_type, .. },
                                } => Value::Variable {
                                    llvm_value,
                                    variable_type: *base_type,
                                },
                                _ => Value::Empty,
                            }
                        }
                        _ => Value::Empty,
                    }
                }
                _ => {
                    self.add_error(String::from("Unsupproted unary operator!"));
                    Value::Empty
                }
            },
            Expression::Index(IndexExpression {
                index_expression,
                index_value,
                ..
            }) => {
                let left = self.gen_expression(&index_expression);
                let right = self.gen_expression(&index_value);

                let base_type = match (&left, &right) {
                    (
                        Value::Variable {
                            variable_type: Type::Array { base_type, .. },
                            ..
                        },
                        Value::Literal { .. },
                    ) => base_type.clone(),
                    _ => {
                        // TODO: ERROR
                        return Value::Empty;
                    }
                };

                let index0 = self.builder.create_literal(&self.builder.get_uint_64(), 0);
                let indicies = [index0, right];

                check!(
                    self,
                    self.builder
                        .create_gep_inbound(&left, *base_type, &indicies),
                    Value
                )
            }
            Expression::Identifier(i) => {
                let str = cast!(&i.token_type, TokenKind::Ident);

                let sym = self.symbol_root.borrow();
                let sym = self.find_up_chain(&sym, &self.current_symbol.borrow(), str);

                match &sym.unwrap().value {
                    SymbolValue::Variable(v) => v.clone(),
                    SymbolValue::Funtion(v) => v.clone(),
                    _ => panic!("sdf"),
                }
            }
            Expression::Literal(literal) => self.gen_literal(literal),
            Expression::IfExpression(IfExpression {
                condition,
                body,
                else_clause,
                ..
            }) => {
                let condition = self.gen_expression(&condition);

                let if_body = check!(
                    self,
                    self.builder.append_block(&self.current_function.borrow()),
                    Value
                );

                if let Some((_, ec)) = else_clause {
                    let else_body = check!(self, self.builder.create_block(), Value);

                    let (end, empty) = match &*self.jump_point.borrow() {
                        Value::Empty => {
                            let end = check!(self, self.builder.create_block(), Value);
                            (end, true)
                        }
                        Value::Block { llvm_value } => (
                            Value::Block {
                                llvm_value: *llvm_value,
                            },
                            false,
                        ),
                        _ => {
                            self.add_error("Unable to get basic block".into());
                            return Value::Empty;
                        }
                    };
                    if empty {
                        self.jump_point.replace(end.clone());
                    }

                    check!(
                        self,
                        self.builder
                            .create_cbranch(&condition, &if_body, &else_body),
                        Value
                    );

                    self.builder.set_position_end(&if_body);
                    let if_ret = self.gen_parse_node(&body);

                    check!(self, self.builder.create_branch(&end), Value);

                    check!(
                        self,
                        self.builder
                            .append_existing_block(&self.current_function.borrow(), &else_body),
                        Value
                    );

                    self.builder.set_position_end(&else_body);
                    let else_ret = self.gen_parse_node(&ec);

                    if empty {
                        check!(
                            self,
                            self.builder
                                .append_existing_block(&self.current_function.borrow(), &end),
                            Value
                        );

                        check!(self, self.builder.create_branch(&end), Value);
                    }

                    self.builder.set_position_end(&end);

                    if empty {
                        self.jump_point.replace(Value::Empty);
                    }

                    match (&if_ret, &else_ret) {
                        (Value::Empty, Value::Empty) => (),
                        (Value::Empty, _) => (),
                        (_, Value::Empty) => (),
                        (a, b) => {
                            let p = check!(
                                self,
                                self.builder.create_phi(a, b, &if_body, &else_body),
                                Value
                            );
                            return p;
                        }
                    }
                } else {
                    let (end, empty) = match &*self.jump_point.borrow() {
                        Value::Empty => {
                            let end = check!(
                                self,
                                self.builder.append_block(&self.current_function.borrow()),
                                Value
                            );
                            (end, true)
                        }
                        Value::Block { llvm_value } => (
                            Value::Block {
                                llvm_value: *llvm_value,
                            },
                            false,
                        ),
                        _ => {
                            self.add_error("Unable to get basic block".into());
                            return Value::Empty;
                        }
                    };
                    if empty {
                        self.jump_point.replace(end.clone());
                    }

                    check!(
                        self,
                        self.builder.create_cbranch(&condition, &if_body, &end),
                        Value
                    );

                    self.builder.set_position_end(&if_body);
                    self.gen_parse_node(&body);
                    check!(self, self.builder.create_branch(&end), Value);

                    self.builder.set_position_end(&end);
                    if empty {
                        self.jump_point.replace(Value::Empty);
                    }
                }

                Value::Empty
            }
            Expression::LoopExpression(LoopExpression { loop_type, .. }) => match loop_type {
                Loop::Until(cond, body) => {
                    let condition_block = check!(
                        self,
                        self.builder.append_block(&self.current_function.borrow()),
                        Value
                    );

                    let body_block = check!(self, self.builder.create_block(), Value);

                    let end = check!(self, self.builder.create_block(), Value);

                    check!(self, self.builder.create_branch(&condition_block), Value);

                    self.builder.set_position_end(&condition_block);
                    let cond = self.gen_expression(cond);

                    check!(
                        self,
                        self.builder.create_cbranch(&cond, &body_block, &end),
                        Value
                    );

                    check!(
                        self,
                        self.builder
                            .append_existing_block(&self.current_function.borrow(), &body_block),
                        Value
                    );

                    self.builder.set_position_end(&body_block);

                    self.gen_parse_node(body);

                    let val = check!(self, self.builder.create_branch(&condition_block), Value);

                    check!(
                        self,
                        self.builder
                            .append_existing_block(&self.current_function.borrow(), &end),
                        Value
                    );

                    self.builder.set_position_end(&end);

                    val
                }
                Loop::Infinite(body) => {
                    let loop_block = check!(
                        self,
                        self.builder.append_block(&self.current_function.borrow()),
                        Value
                    );
                    check!(self, self.builder.create_branch(&loop_block), Value);

                    self.builder.set_position_end(&loop_block);

                    self.gen_parse_node(body);

                    check!(self, self.builder.create_branch(&loop_block), Value)
                }
            },
            Expression::FunctionCall(FunctionCall {
                arguments,
                expression_to_call,
                generic,
                ..
            }) => {
                let arguments: Vec<_> = arguments.iter().map(|f| self.gen_expression(f)).collect();
                let expr = self.gen_expression(&expression_to_call);
                if let Value::FunctionTemplate {
                    body,
                    path,
                    ty,
                    ty_params,
                    existing,
                    specialization,
                } = &expr
                {
                    let old_block = self.current_block.take();

                    let old_symbol = self.current_symbol.take();
                    let old_function = self.current_function.take();
                    self.current_symbol.replace(path.clone());

                    let mut fn_types = vec![];

                    // Generate type arguments
                    {
                        let mut sym = self.symbol_root.borrow_mut();
                        let current = self.get_symbol_mut(&mut sym, &self.current_symbol.borrow());

                        if let Some(current) = current {
                            let mut gens = generic
                                .as_ref()
                                .map_or(Some([].iter()), |f| Some(f.iter()))
                                .unwrap();
                            let mut pram_iter = ty_params.iter();

                            while let Some(p) = gens.next() {
                                let (name, bounds) = pram_iter.next().unwrap();
                                if let Some(sym) = current.children.get_mut(name) {
                                    match &mut sym.value {
                                        SymbolValue::Generic(ty, _) => {
                                            *ty = self.gen_type(p);
                                            fn_types.push(ty.to_string())
                                        }
                                        _ => (),
                                    }
                                }
                            }

                            pram_iter.for_each(|(name, bounds)| {
                                let found = ty.parameters.iter().position(|f| {
                                    if let dsl_lexer::ast::Type::NamedType(t) = &f.symbol_type {
                                        if cast!(&t.token_type, TokenKind::Ident) == name {
                                            return true;
                                        }
                                    }
                                    return false;
                                });
                                if let Some(pos) = found {
                                    if let Some(sym) = current.children.get_mut(name) {
                                        match &mut sym.value {
                                            SymbolValue::Generic(ty, _) => {
                                                *ty = arguments[pos].get_type().clone();
                                                fn_types.push(ty.to_string())
                                            }
                                            _ => (),
                                        }
                                    }
                                } else {
                                    self.add_error(format!(
                                        "Unable to determine type for generic paramater: `{}` -- Please provide type parameter",
                                        name
                                    ))
                                }
                            })
                        }
                    }

                    let specialization = specialization.iter().find(|(f, _)| {
                        for (a, b) in f.iter().zip(fn_types.iter()) {
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
                    let existing_impl = existing.iter().find(|(f, _)| *f == &fn_types);

                    let name = if let Some(last) = path.last() {
                        if let Some(ty) = existing_impl {
                            Some(ty.1.last().unwrap().clone())
                        } else {
                            let (return_type, types) = if let Some(special) = specialization {
                                let mut sym = self.symbol_root.borrow();
                                let current = self.get_symbol(&mut sym, &special.1);

                                if let Some(Symbol {
                                    value:
                                        SymbolValue::Funtion(Value::FunctionTemplate {
                                            body, ty, ..
                                        }),
                                    ..
                                }) = current
                                {
                                    let return_type = self.gen_type(&ty.return_type);

                                    let types: Vec<(String, Type)> = ty
                                        .parameters
                                        .iter()
                                        .map(|f| {
                                            (
                                                cast!(&f.symbol.token_type, TokenKind::Ident)
                                                    .clone(),
                                                self.gen_type(&f.symbol_type),
                                            )
                                        })
                                        .collect();
                                    (return_type, types)
                                } else {
                                    let return_type = self.gen_type(&ty.return_type);

                                    let types: Vec<(String, Type)> = ty
                                        .parameters
                                        .iter()
                                        .map(|f| {
                                            (
                                                cast!(&f.symbol.token_type, TokenKind::Ident)
                                                    .clone(),
                                                self.gen_type(&f.symbol_type),
                                            )
                                        })
                                        .collect();
                                    (return_type, types)
                                }
                            } else {
                                let return_type = self.gen_type(&ty.return_type);

                                let types: Vec<(String, Type)> = ty
                                    .parameters
                                    .iter()
                                    .map(|f| {
                                        (
                                            cast!(&f.symbol.token_type, TokenKind::Ident).clone(),
                                            self.gen_type(&f.symbol_type),
                                        )
                                    })
                                    .collect();
                                (return_type, types)
                            };
                            let path = &path[..path.len() - 1];
                            let name = self.get_next_name(path, last.clone());

                            let mut typesda = Vec::from(path);
                            typesda.push(name.clone());

                            let function_type = self.builder.get_fn(return_type.clone(), &types);

                            let function = check!(
                                self,
                                self.builder.add_function(
                                    function_type,
                                    name.to_string(),
                                    self.module
                                ),
                                Value
                            );

                            let block = check!(self, self.builder.append_block(&function), Value);

                            self.current_block.replace(block);
                            self.builder.set_position_end(&self.current_block.borrow());

                            self.current_function.replace(function.clone());

                            self.add_and_set_symbol_from_path(
                                &path,
                                &name,
                                SymbolValue::Funtion(function),
                            );

                            let pallocs: Result<Vec<Value>, _> = types
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

                            {
                                let mut cur_sym = self.symbol_root.borrow_mut();
                                let current = self
                                    .get_symbol_mut(&mut cur_sym, &self.current_symbol.borrow());

                                if let Some(c) = current {
                                    for ((name, ty), alloc) in types.iter().zip(pallocs.into_iter())
                                    {
                                        c.add_child(&name, SymbolValue::Variable(alloc));
                                    }
                                }
                            }

                            let alloc = match return_type {
                                Type::Unit { .. } => None,
                                ty => Some(check!(self, self.builder.create_alloc(&ty), Value)),
                            };

                            let val = if let Some(special) = specialization {
                                let mut sym = self.symbol_root.borrow();
                                let current = self.get_symbol(&mut sym, &special.1);

                                if let Some(Symbol {
                                    value:
                                        SymbolValue::Funtion(Value::FunctionTemplate {
                                            body, ty, ..
                                        }),
                                    ..
                                }) = current
                                {
                                    self.gen_parse_node(body.as_ref())
                                } else {
                                    self.gen_parse_node(body.as_ref())
                                }
                            } else {
                                self.gen_parse_node(body.as_ref())
                            };

                            if let Some(alloc) = alloc {
                                if val.has_value() {
                                    check!(self, self.builder.create_store(&alloc, &val), Value);
                                }

                                check!(self, self.builder.create_ret(&alloc), Value);
                            } else {
                                self.builder.create_ret_void();
                            }

                            Some(name)
                        }
                    } else {
                        None
                    };

                    self.current_block.replace(old_block);
                    self.current_symbol.replace(old_symbol);
                    self.current_function.replace(old_function);

                    self.builder.set_position_end(&self.current_block.borrow());

                    if let Some(name) = name {
                        let mut npath = Vec::from(&path[..path.len() - 1]);
                        npath.push(name);

                        let mut sym = self.symbol_root.borrow_mut();

                        // let mut sym = self.symbol_root.borrow_mut();
                        let fn_templ = self.get_symbol_mut(&mut sym, &path);
                        if let Some(Symbol {
                            value: SymbolValue::Funtion(Value::FunctionTemplate { existing, .. }),
                            ..
                        }) = fn_templ
                        {
                            existing.insert(fn_types, npath.clone());
                        }

                        let current = self.get_symbol_mut(&mut sym, &npath);
                        if let Some(Symbol {
                            value: SymbolValue::Funtion(val),
                            ..
                        }) = current
                        {
                            check!(self, self.builder.create_fn_call(val, arguments), Value)
                        } else {
                            Value::Empty
                        }
                    } else {
                        Value::Empty
                    }
                } else {
                    check!(self, self.builder.create_fn_call(&expr, arguments), Value)
                }
            }
            Expression::Block(values, _) => {
                return values
                    .iter()
                    .map(|stmt| self.gen_parse_node(stmt))
                    .last()
                    .or(Some(Value::Empty))
                    .unwrap()
            }
            _ => {
                self.add_error(String::from("Unsupported expression"));
                Value::Empty
            }
        }
    }
}
