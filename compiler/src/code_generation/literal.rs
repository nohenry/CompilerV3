use llvm_sys::core::{LLVMConstInt, LLVMConstReal, LLVMFloatType, LLVMInt1Type, LLVMInt32Type};

use crate::ast::Literal;

use super::{module::Module, Type, Value};

impl Module {
    pub(super) fn gen_literal(&self, literal: &Literal) -> Value {
        match literal {
            Literal::Integer(i, ..) => Value::new(
                unsafe { LLVMConstInt(LLVMInt32Type(), *i, 0) },
                Box::new(Type::new(unsafe { LLVMInt32Type() }, false)),
            ),
            Literal::Float(f, ..) => Value::new(
                unsafe { LLVMConstReal(LLVMFloatType(), *f) },
                Box::new(Type::new(unsafe { LLVMFloatType() }, false)),
            ),
            Literal::Boolean(b, ..) => Value::new(
                unsafe { LLVMConstInt(LLVMInt1Type(), if *b { 1 } else { 0 }, 0) },
                Box::new(Type::new(unsafe { LLVMInt1Type() }, false)),
            ),
            _ => {
                self.add_error(String::from("Unsupported literal"));
                Value::empty()
            }
        }
    }
}
