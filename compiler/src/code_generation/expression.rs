use llvm_sys::{
    c_str,
    core::{
        LLVMBuildBinOp, LLVMBuildInBoundsGEP2, LLVMBuildLoad2, LLVMBuildStore, LLVMConstInt,
        LLVMInt64Type,
    },
    LLVMOpcode,
};

use crate::{
    ast::{BinaryExpression, Expression, ExpressionIndex},
    cast,
    lexer::TokenKind,
    util::NULL_STR,
};

use super::{module::Module, SymbolValue, Type, Value};

impl Module {
    pub(super) fn gen_expression(&self, expression: &Expression) -> Value {
        match expression {
            Expression::BinaryExpression(BinaryExpression {
                left,
                operator,
                right,
                ..
            }) => {
                use crate::lexer::OperatorKind::*;
                let func = match operator {
                    Assignment => {
                        let left = self.gen_expression_ref(left);
                        let right = self.gen_expression(right);
                        match (left, right) {
                            (
                                Value::Variable {
                                    llvm_value: lvalue, ..
                                },
                                Value::Literal {
                                    llvm_value: rvalue, ..
                                },
                            ) => unsafe {
                                LLVMBuildStore(self.builder, rvalue, lvalue);
                            },
                            _ => (),
                        }

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
                                self.add_error(format!("Unsupported binary operation {:?}", c));
                                return Value::Empty;
                            }
                        };

                        let left = self.gen_expression_ref(left);
                        let right = self.gen_expression(right);
                        match (left, right) {
                            (
                                Value::Variable {
                                    llvm_value: lvalue,
                                    variable_type: ltype,
                                },
                                Value::Literal {
                                    llvm_value: rvalue, ..
                                },
                            ) => unsafe {
                                let load = LLVMBuildLoad2(
                                    self.builder,
                                    ltype.get_type(),
                                    lvalue,
                                    NULL_STR,
                                );
                                let result =
                                    LLVMBuildBinOp(self.builder, oper, load, rvalue, NULL_STR);
                                LLVMBuildStore(self.builder, result, lvalue);
                            },
                            _ => (),
                        }
                        // load modify and store value for op=
                        return Value::Empty;
                    }
                };

                let left = self.gen_expression(left);
                let right = self.gen_expression(right);

                let (value, vtype) = match (left, right) {
                    (
                        Value::Literal {
                            llvm_value: lvalue,
                            literal_type: ltype,
                        },
                        Value::Literal {
                            llvm_value: rvalue, ..
                        },
                    ) => unsafe {
                        (
                            LLVMBuildBinOp(self.builder, func, lvalue, rvalue, c_str!("")),
                            ltype,
                        )
                    },
                    _ => {
                        // TODO: ERROR
                        return Value::Empty;
                    }
                };

                Value::Literal {
                    llvm_value: value,
                    literal_type: vtype,
                }
            }
            Expression::Index(ExpressionIndex {
                index_expression,
                index_value,
                ..
            }) => {
                let left = self.gen_expression_ref(&index_expression);
                let right = self.gen_expression(&index_value);

                let (ivalue, lvalue, ltype, base_type) = match (left, right) {
                    (
                        Value::Variable {
                            llvm_value: lvalue,
                            variable_type:
                                Type::Array {
                                    llvm_type: ltype,
                                    base_type,
                                },
                        },
                        Value::Literal {
                            llvm_value: rvalue, ..
                        },
                    ) => (rvalue, lvalue, ltype, base_type),
                    _ => {
                        // TODO: ERROR
                        return Value::Empty;
                    }
                };

                let value = unsafe {
                    let index0 = LLVMConstInt(LLVMInt64Type(), 0, 0);
                    let mut indicies = [index0, ivalue];

                    LLVMBuildInBoundsGEP2(
                        self.builder,
                        ltype,
                        lvalue,
                        indicies.as_mut_ptr(),
                        2,
                        NULL_STR,
                    )

                    // LLVMBuildLoad2(self.builder, base_type.get_type(), ptr, c_str!("Index"))
                };

                Value::Variable {
                    llvm_value: value,
                    variable_type: *base_type,
                }
            }
            Expression::Literal(literal) => self.gen_literal(literal),
            _ => {
                self.add_error(String::from("Unsupported expression"));
                Value::Empty
            }
        }
    }

    pub(super) fn gen_expression_ref(&self, expression: &Expression) -> Value {
        match expression {
            Expression::Identifier(i) => {
                let str = cast!(&i.token_type, TokenKind::Ident);

                let sym = self.symbol_root.borrow();
                let sym = self.find_up_chain(&sym, &self.current_symbol, str);

                match &sym.unwrap().value {
                    SymbolValue::Variable(v) => v.clone(),
                    _ => panic!("sdf"),
                }
            }
            _ => {
                self.add_error(String::from("Unsupported expression"));
                Value::Empty
            }
        }
    }
}
