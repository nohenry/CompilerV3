use llvm_sys::core::{
    LLVMArrayType, LLVMFloatType, LLVMInt1Type, LLVMInt32Type, LLVMIntType, LLVMVoidType,
};

use dsl_lexer::ast::{ArrayInitializer, Expression, Literal, Type};

use super::module::Module;

impl Module {
    pub(super) fn gen_type(&self, _type: &Type) -> dsl_symbol::Type {
        match _type {
            Type::Int(s, _) => unsafe {
                dsl_symbol::Type::Integer {
                    llvm_type: LLVMIntType(*s as _),
                    signed: false,
                }
            },
            Type::Unit => unsafe {
                dsl_symbol::Type::Unit {
                    llvm_type: LLVMVoidType(),
                }
            },
            c => {
                self.add_error(format!("Unsupported type {:?}", c));
                dsl_symbol::Type::Empty
            }
        }
    }

    pub(super) fn gen_literal_type(&self, literal: &Literal) -> dsl_symbol::Type {
        match literal {
            Literal::Integer(..) => dsl_symbol::Type::Integer {
                llvm_type: unsafe { LLVMInt32Type() },
                signed: false,
            },
            Literal::Float(..) => dsl_symbol::Type::Float {
                llvm_type: unsafe { LLVMFloatType() },
            },
            Literal::Boolean(..) => dsl_symbol::Type::Boolean {
                llvm_type: unsafe { LLVMInt1Type() },
            },
            Literal::Array(ArrayInitializer { elements, .. }) => {
                let ttype = self.gen_node_type(&elements[0], false);
                let array_type = unsafe { LLVMArrayType(ttype.get_type(), elements.len() as _) };

                dsl_symbol::Type::Array {
                    llvm_type: array_type,
                    base_type: Box::new(ttype),
                }
            }
            _ => {
                self.add_error(String::from("Unable to generate type from expression"));
                dsl_symbol::Type::Empty
            }
        }
    }

    pub(super) fn gen_node_type(
        &self,
        expression: &Expression,
        supress_error: bool,
    ) -> dsl_symbol::Type {
        match expression {
            Expression::Literal(literal) => self.gen_literal_type(literal),
            _ => {
                if !supress_error {
                    self.add_error(String::from("Unable to generate type from expression"));
                }
                dsl_symbol::Type::Empty
            }
        }
    }
}
