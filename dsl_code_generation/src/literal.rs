use llvm_sys::core::{
    LLVMArrayType, LLVMConstArray, LLVMConstInt, LLVMConstReal, LLVMFloatType, LLVMInt1Type,
    LLVMInt32Type,
};

use dsl_lexer::ast::{ArrayInitializer, Literal};

use super::{module::Module};
use dsl_symbol::{ Type, Value};

impl Module {
    pub(super) fn gen_literal(&self, literal: &Literal) -> Value {
        match literal {
            Literal::Integer(i, ..) => Value::Literal {
                llvm_value: unsafe { LLVMConstInt(LLVMInt32Type(), *i, 0) },
                literal_type: Type::Integer {
                    llvm_type: unsafe { LLVMInt32Type() },
                    signed: false,
                },
            },
            Literal::Float(f, ..) => Value::Literal {
                llvm_value: unsafe { LLVMConstReal(LLVMFloatType(), *f) },
                literal_type: Type::Float {
                    llvm_type: unsafe { LLVMFloatType() },
                },
            },
            Literal::Boolean(b, ..) => Value::Literal {
                llvm_value: unsafe { LLVMConstInt(LLVMInt1Type(), if *b { 1 } else { 0 }, 0) },
                literal_type: Type::Boolean {
                    llvm_type: unsafe { LLVMInt1Type() },
                },
            },
            Literal::Array(ArrayInitializer { elements, .. }) => {
                let elements: Vec<_> = elements.iter().map(|f| self.gen_expression(f)).collect();
                let atype = elements.iter().all(|f| match (f, &elements[0]) {
                    (
                        Value::Literal { literal_type, .. },
                        Value::Literal {
                            literal_type: ttype,
                            ..
                        },
                    ) => literal_type == ttype,
                    _ => false,
                });
                if !atype {
                    self.add_error(String::from(
                        "Array initializer values don't match or aren't constant",
                    ));
                    return Value::Empty;
                }
                let atype = match &elements[0] {
                    Value::Literal { literal_type, .. } => literal_type.clone(),
                    _ => panic!("Should have the right type"),
                };

                let mut values: Vec<_> = elements
                    .iter()
                    .map(|f| match f {
                        Value::Literal { llvm_value, .. } => *llvm_value,
                        _ => panic!("Unable to create array initializer"),
                    })
                    .collect();

                let llvm_value = unsafe {
                    LLVMConstArray(
                        atype.get_type(),
                        values.as_mut_ptr(),
                        values.len().try_into().unwrap(),
                    )
                };
                let llvm_type =
                    unsafe { LLVMArrayType(atype.get_type(), values.len().try_into().unwrap()) };

                Value::Literal {
                    llvm_value,
                    literal_type: Type::Array {
                        llvm_type,
                        base_type: Box::new(atype),
                    },
                }
            }
            _ => {
                self.add_error(String::from("Unsupported literal"));
                Value::Empty
            }
        }
    }
}
