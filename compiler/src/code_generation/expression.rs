use std::sync::mpsc::channel;

use llvm_sys::{
    c_str,
    core::{LLVMBuildBinOp, LLVMBuildStore},
    LLVMOpcode,
};

use crate::{
    ast::{BinaryExpression, Expression},
    cast,
    lexer::TokenKind,
};

use super::{module::Module, SymbolValue, Value};

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
                        unsafe {
                            LLVMBuildStore(self.builder, right.llvm_value, left.llvm_value);
                        }
                        return Value::empty();
                    }
                    Plus => LLVMOpcode::LLVMAdd,
                    Minus => LLVMOpcode::LLVMSub,
                    Mult => LLVMOpcode::LLVMMul,
                    Divide => LLVMOpcode::LLVMSDiv,
                    BitAnd => LLVMOpcode::LLVMAnd,
                    BitOr => LLVMOpcode::LLVMOr,
                    BitXor => LLVMOpcode::LLVMXor,
                    Percent => LLVMOpcode::LLVMSRem,
                    c => {
                        self.add_error(format!("Unsupported binary operation {:?}", c));
                        return Value::empty();
                    }
                };

                let left = self.gen_expression(left);
                let right = self.gen_expression(right);

                let value = unsafe {
                    LLVMBuildBinOp(
                        self.builder,
                        func,
                        left.llvm_value,
                        right.llvm_value,
                        c_str!(""),
                    )
                };

                Value::new(value, left.value_type)
            }
            Expression::Literal(literal) => self.gen_literal(literal),
            _ => {
                self.add_error(String::from("Unsupported expression"));
                Value::empty()
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
                Value::empty()
            }
        }
    }
}
