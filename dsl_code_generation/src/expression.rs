use std::ffi::{CStr, CString};

use dsl_errors::go;
use llvm_sys::{LLVMIntPredicate, LLVMOpcode, LLVMRealPredicate};

use dsl_lexer::ast::{
    BinaryExpression, Expression, IndexExpression, IfExpression, UnaryExpression,
};
use dsl_lexer::{OperatorKind, TokenKind};
use dsl_util::{c_str, cast, NULL_STR};
use llvm_sys::core::{
    LLVMAppendBasicBlock, LLVMAppendExistingBasicBlock, LLVMCreateBasicBlockInContext,
    LLVMGetGlobalContext, LLVMGetMDKindID, LLVMMDString, LLVMSetMetadata,
};

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

                        go!(self, self.builder.create_store(&left, &right), Value);

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
                                    Type::Integer { .. } | Type::Boolean { .. } => {
                                        let func = match c {
                                            Eq => LLVMIntPredicate::LLVMIntEQ,
                                            _ => {
                                                self.add_error(format!(
                                                    "Unsupported binary operation {:?}",
                                                    c
                                                ));
                                                return Value::Empty;
                                            }
                                        };
                                        return go!(
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
                                        return go!(
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

                        let op = go!(self, self.builder.create_bin_op(&left, &right, oper), Value);
                        go!(self, self.builder.create_store(&left, &op), Value);

                        // load modify and store value for op=
                        return Value::Empty;
                    }
                };

                let left = self.gen_expression(left);
                let right = self.gen_expression(right);

                go!(self, self.builder.create_bin_op(&left, &right, func), Value)
            }
            Expression::UnaryExpression(UnaryExpression {
                expression,
                operator,
                ..
            }) => match operator {
                OperatorKind::Minus => {
                    let expr = self.gen_expression(&expression);

                    go!(self, self.builder.create_neg(&expr), Value)
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
                            let value = go!(self, self.builder.create_load(&value), Value);

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

                go!(
                    self,
                    self.builder
                        .create_gep_inbound(&left, *base_type, &indicies),
                    Value
                )
            }
            Expression::Identifier(i) => {
                let str = cast!(&i.token_type, TokenKind::Ident);

                let sym = self.symbol_root.borrow();
                let sym = self.find_up_chain(&sym, &self.current_symbol, str);

                match &sym.unwrap().value {
                    SymbolValue::Variable(v) => v.clone(),
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

                let if_body = unsafe {
                    LLVMAppendBasicBlock(self.current_function.borrow().as_mut().unwrap(), NULL_STR)
                };

                if let Some((_, ec)) = else_clause {
                    let else_body = unsafe {
                        LLVMCreateBasicBlockInContext(LLVMGetGlobalContext(), NULL_STR)
                        // LLVMAppendBasicBlock(
                        //     self.current_function.borrow().as_mut().unwrap(),
                        //     NULL_STR,
                        // )
                    };

                    let (end, empty) = match *self.jump_point.borrow() {
                        Value::Empty => unsafe {
                            let end =
                                LLVMCreateBasicBlockInContext(LLVMGetGlobalContext(), NULL_STR);
                            (end, true)
                        },
                        Value::Block { llvm_value } => (llvm_value, false),
                        _ => {
                            self.add_error("Unable to get basic block".into());
                            return Value::Empty;
                        }
                    };
                    if empty {
                        self.jump_point.replace(Value::Block { llvm_value: end });
                    }

                    go!(
                        self,
                        self.builder.create_cbranch(&condition, if_body, else_body),
                        Value
                    );

                    self.builder.set_position_end(if_body);
                    self.gen_parse_node(&body);

                    go!(self, self.builder.create_branch(end), Value);

                    unsafe {
                        LLVMAppendExistingBasicBlock(
                            self.current_function.borrow().as_mut().unwrap(),
                            else_body,
                        );
                    }

                    self.builder.set_position_end(else_body);
                    self.gen_parse_node(&ec);
                    let brn = go!(self, self.builder.create_branch(end), Value);
                    match brn {
                        Value::Instruction { llvm_value } => unsafe {
                            let strds = LLVMMDString(
                                CString::new("Hello".to_string()).unwrap().as_ptr(),
                                5,
                            );
                            LLVMSetMetadata(llvm_value, LLVMGetMDKindID(c_str!("Potato"), 6), strds)
                        },
                        _ => (),
                    }

                    if empty {
                        unsafe {
                            LLVMAppendExistingBasicBlock(
                                self.current_function.borrow().as_mut().unwrap(),
                                end,
                            );
                        }
                    }
                    self.builder.set_position_end(end);

                    if empty {
                        self.jump_point.replace(Value::Empty);
                    }
                } else {
                    // TODO: remove ffi function
                    let (end, empty) = match *self.jump_point.borrow() {
                        Value::Empty => unsafe {
                            let end = LLVMAppendBasicBlock(
                                self.current_function.borrow().as_mut().unwrap(),
                                NULL_STR,
                            );
                            (end, true)
                        },
                        Value::Block { llvm_value } => (llvm_value, false),
                        _ => {
                            self.add_error("Unable to get basic block".into());
                            return Value::Empty;
                        }
                    };
                    if empty {
                        self.jump_point.replace(Value::Block { llvm_value: end });
                    }

                    go!(
                        self,
                        self.builder.create_cbranch(&condition, if_body, end),
                        Value
                    );

                    self.builder.set_position_end(if_body);
                    self.gen_parse_node(&body);
                    go!(self, self.builder.create_branch(end), Value);

                    self.builder.set_position_end(end);
                    if empty {
                        self.jump_point.replace(Value::Empty);
                    }
                }

                Value::Empty
            }
            _ => {
                self.add_error(String::from("Unsupported expression"));
                Value::Empty
            }
        }
    }
}
