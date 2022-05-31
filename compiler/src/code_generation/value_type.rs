use llvm_sys::core::{
    LLVMArrayType, LLVMFloatType, LLVMInt1Type, LLVMInt32Type, LLVMIntType, LLVMVoidType,
};

use crate::ast::{ArrayInitializer, Expression, Literal, Type};

use super::module::Module;

impl Module {
    pub(super) fn gen_type(&self, _type: &Type) -> super::Type {
        match _type {
            Type::Int(s, _) => unsafe {
                super::Type::Integer {
                    llvm_type: LLVMIntType(*s as _),
                    signed: false,
                }
            },
            Type::Unit => unsafe {
                super::Type::Unit {
                    llvm_type: LLVMVoidType(),
                }
            },
            c => {
                self.add_error(format!("Unsupported type {:?}", c));
                super::Type::Empty
            }
        }
    }

    pub(super) fn gen_literal_type(&self, literal: &Literal) -> super::Type {
        match literal {
            Literal::Integer(..) => super::Type::Integer {
                llvm_type: unsafe { LLVMInt32Type() },
                signed: false,
            },
            Literal::Float(..) => super::Type::Float {
                llvm_type: unsafe { LLVMFloatType() },
            },
            Literal::Boolean(..) => super::Type::Boolean {
                llvm_type: unsafe { LLVMInt1Type() },
            },
            Literal::Array(ArrayInitializer { elements, .. }) => {
                let ttype = self.gen_node_type(&elements[0], false);
                let array_type = unsafe { LLVMArrayType(ttype.get_type(), elements.len() as _) };

                super::Type::Array {
                    llvm_type: array_type,
                    base_type: Box::new(ttype),
                }
            }
            _ => {
                self.add_error(String::from("Unable to generate type from expression"));
                super::Type::Empty
            }
        }
    }

    pub(super) fn gen_node_type(
        &self,
        expression: &Expression,
        supress_error: bool,
    ) -> super::Type {
        match expression {
            Expression::Literal(literal) => self.gen_literal_type(literal),
            _ => {
                if !supress_error {
                    self.add_error(String::from("Unable to generate type from expression"));
                }
                super::Type::Empty
            }
        }
    }
}
