use dsl_symbol::{Symbol, SymbolValue};
use dsl_util::cast;
use linked_hash_map::LinkedHashMap;
use llvm_sys::core::{
    LLVMArrayType, LLVMDoubleType, LLVMFloatType, LLVMFunctionType, LLVMInt1Type, LLVMInt32Type,
    LLVMInt8Type, LLVMIntType, LLVMPointerType, LLVMVoidType,
};

use dsl_lexer::{
    ast::{
        ArrayInitializer, ArrayType, Expression, FunctionType, GenericType, Literal, ReferenceType,
        Type,
    },
    TokenKind,
};

use super::module::Module;

impl Module {
    pub(super) fn gen_type(&self, _type: &Type) -> dsl_symbol::Type {
        match _type {
            Type::Int(s, _) => unsafe {
                dsl_symbol::Type::Integer {
                    llvm_type: LLVMIntType(*s as _),
                    signed: true,
                }
            },
            Type::Uint(s, _) => unsafe {
                dsl_symbol::Type::Integer {
                    llvm_type: LLVMIntType(*s as _),
                    signed: false,
                }
            },
            Type::Float(s, _) => unsafe {
                match *s {
                    32 => dsl_symbol::Type::Float {
                        llvm_type: LLVMFloatType(),
                    },
                    64 => dsl_symbol::Type::Float {
                        llvm_type: LLVMDoubleType(),
                    },
                    _ => dsl_symbol::Type::Empty,
                }
            },
            Type::FunctionType(FunctionType {
                parameters,
                return_type,
                ..
            }) => {
                let return_type = self.gen_type(&return_type);
                let parameters: Vec<_> = parameters.iter().map(|ty| self.gen_type(ty)).collect();
                let mut llvm_parameters: Vec<_> = parameters.iter().map(|f| f.get_type()).collect();

                let mut params = LinkedHashMap::new();
                for p in parameters {
                    params.insert("".to_string(), p);
                }

                dsl_symbol::Type::Function {
                    llvm_type: unsafe {
                        LLVMFunctionType(
                            return_type.get_type(),
                            llvm_parameters.as_mut_ptr(),
                            llvm_parameters.len().try_into().unwrap(),
                            0,
                        )
                    },
                    return_type: Box::new(return_type),
                    parameters: params,
                }
            }
            Type::Bool(_) => unsafe {
                dsl_symbol::Type::Boolean {
                    llvm_type: LLVMInt1Type(),
                }
            },
            Type::Char(_) => unsafe {
                dsl_symbol::Type::Char {
                    llvm_type: LLVMInt8Type(),
                }
            },
            Type::ArrayType(ArrayType {
                base_type, size, ..
            }) => {
                if let Some((_, size)) = size {
                    let ty = self.gen_type(base_type.as_ref());
                    unsafe {
                        dsl_symbol::Type::Array {
                            llvm_type: LLVMArrayType(ty.get_type(), (*size).try_into().unwrap()),
                            base_type: Box::new(ty),
                        }
                    }
                } else {
                    dsl_symbol::Type::Empty
                }
            }
            Type::ReferenceType(ReferenceType { base_type, .. }) => {
                let ty = self.gen_type(base_type.as_ref());
                unsafe {
                    dsl_symbol::Type::Reference {
                        llvm_type: LLVMPointerType(ty.get_type(), 0),
                        base_type: Box::new(ty),
                    }
                }
            }
            Type::Unit => unsafe {
                dsl_symbol::Type::Unit {
                    llvm_type: LLVMVoidType(),
                }
            },
            c @ Type::NamedType(token) => {
                let str = cast!(&token.token_type, TokenKind::Ident);

                let sym = self.symbol_root.borrow();
                {
                    let current = self.get_symbol(&sym, &self.current_symbol.borrow());

                    if let Some(current) = current {
                        if let Some(al_sym) = current.children.get(str) {
                            let syysdf = al_sym;
                            match &syysdf.value {
                                SymbolValue::Generic(ty, bounds) => return ty.clone(),
                                SymbolValue::Template(ty) => return ty.clone(),
                                _ => {
                                    self.add_error(format!("Unsupported type {:?}", c));
                                    return dsl_symbol::Type::Empty;
                                }
                            }
                        }
                    }
                }

                if let Some(Symbol { value, .. }) =
                    self.find_up_chain(&sym, &self.current_symbol.borrow(), str)
                {
                    match value {
                        SymbolValue::Template(ty) => return ty.clone(),
                        SymbolValue::Action(ty) => return ty.clone(),
                        SymbolValue::Spec(ty) => return ty.clone(),
                        _ => {
                            self.add_error(format!("Unsupported type {:?}", c));
                            return dsl_symbol::Type::Empty;
                        }
                    }
                }

                dsl_symbol::Type::Empty
            }
            Type::GenericType(GenericType {
                base_type,
                arguments,
                ..
            }) => dsl_symbol::Type::Generic {
                base_type: Box::new(self.gen_type(base_type)),
                parameters: arguments.iter().map(|f| self.gen_type(f)).collect(),
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
