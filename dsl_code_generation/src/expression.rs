use dsl_errors::go;
use llvm_sys::{
    core::{
        LLVMBuildBinOp, LLVMBuildInBoundsGEP2, LLVMBuildLoad2, LLVMBuildNeg, LLVMBuildStore,
        LLVMConstInt, LLVMInt64Type,
    },
    LLVMOpcode,
};

use dsl_lexer::ast::{BinaryExpression, Expression, ExpressionIndex, UnaryExpression};
use dsl_lexer::{OperatorKind, TokenKind};
use dsl_util::{c_str, cast, NULL_STR};

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
                                self.add_error(format!("Unsupported binary operation {:?}", c));
                                return Value::Empty;
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
                    match expr {
                        Value::Literal { llvm_value, .. } => unsafe {
                            // LLVMBuildNeg(self.builder, llvm_value, NULL_STR)
                            Value::Empty
                        },
                        _ => Value::Empty,
                    }
                }
                _ => {
                    self.add_error(String::from("Unsupproted unary operator!"));
                    Value::Empty
                }
            },
            Expression::Index(ExpressionIndex {
                index_expression,
                index_value,
                ..
            }) => {
                let left = self.gen_expression(&index_expression);
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
                };

                Value::Variable {
                    llvm_value: value,
                    variable_type: *base_type,
                }
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
            _ => {
                self.add_error(String::from("Unsupported expression"));
                Value::Empty
            }
        }
    }
}
