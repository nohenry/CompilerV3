use dsl_errors::check;
use llvm_sys::{LLVMIntPredicate, LLVMOpcode, LLVMRealPredicate};

use dsl_lexer::ast::{
    BinaryExpression, Expression, FunctionCall, IfExpression, IndexExpression, Loop,
    LoopExpression, UnaryExpression,
};
use dsl_lexer::{OperatorKind, TokenKind};
use dsl_util::cast;

use super::module::Module;
use dsl_symbol::{SymbolValue, Type, Value};

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

                        let op = check!(self, self.builder.create_bin_op(&left, &right, oper), Value);
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
                ..
            }) => {
                let arguments: Vec<_> = arguments.iter().map(|f| self.gen_expression(f)).collect();
                let expr = self.gen_expression(&expression_to_call);
                check!(self, self.builder.create_fn_call(&expr, arguments), Value)
            }
            Expression::Block(values, _) => {
                return values
                    .iter()
                    .map(|stmt| self.gen_parse_node(stmt))
                    .last()
                    .unwrap()
            }
            _ => {
                self.add_error(String::from("Unsupported expression"));
                Value::Empty
            }
        }
    }
}
