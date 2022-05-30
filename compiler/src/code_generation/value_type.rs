use llvm_sys::core::{
    LLVMArrayType, LLVMFloatType, LLVMInt1Type, LLVMInt32Type, LLVMIntType, LLVMVoidType,
};

use crate::ast::{ArrayInitializer, Expression, Literal, Type};

use super::module::Module;

impl Module {
    pub(super) fn gen_type(&self, _type: &Type) -> super::Type {
        match _type {
            Type::Int(s, _) => unsafe { super::Type::new(LLVMIntType(*s as _), true) },
            Type::Unit => unsafe { super::Type::new(LLVMVoidType(), false) },
            c => {
                self.add_error(format!("Unsupported type {:?}", c));
                super::Type::empty()
            }
        }
    }

    pub(super) fn gen_literal_type(&self, literal: &Literal) -> super::Type {
        match literal {
            Literal::Integer(..) => super::Type::new(unsafe { LLVMInt32Type() }, false),
            Literal::Float(..) => super::Type::new(unsafe { LLVMFloatType() }, false),
            Literal::Boolean(..) => super::Type::new(unsafe { LLVMInt1Type() }, false),
            Literal::Array(ArrayInitializer { elements, .. }) => {
                let array_type = unsafe {
                    LLVMArrayType(
                        self.gen_node_type(&elements[0]).llvm_type,
                        elements.len() as _,
                    )
                };
                super::Type::new(array_type, false)
            }
            _ => {
                self.add_error(String::from("Unable to generate type from expression"));
                super::Type::empty()
            }
        }
    }

    pub(super) fn gen_node_type(&self, expression: &Expression) -> super::Type {
        match expression {
            Expression::Literal(literal) => self.gen_literal_type(literal),
            _ => {
                self.add_error(String::from("Unable to generate type from expression"));
                super::Type::empty()
            }
        }
    }
}
