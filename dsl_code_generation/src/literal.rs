use std::collections::HashMap;

use dsl_util::NULL_STR;
use llvm_sys::{core::{
    LLVMArrayType, LLVMConstArray, LLVMConstInt, LLVMConstReal, LLVMConstString, 
    LLVMFloatType, LLVMInt1Type, LLVMInt32Type, LLVMInt8Type, LLVMBuildGlobalString,
}, LLVMCreateStringLiteral};

use dsl_lexer::ast::{ArrayInitializer, Literal, TemplateInitializer};

use super::module::Module;
use dsl_symbol::{Type, Value};

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
            Literal::String(s, _) => Value::Literal {
                llvm_value: unsafe {
                    LLVMCreateStringLiteral(self.builder.get_builder(), s.as_ptr() as *const _, s.len().try_into().unwrap(), NULL_STR)
                },
                literal_type: Type::String {
                    llvm_type: unsafe {
                        LLVMArrayType(LLVMInt8Type(), (s.len() + 1).try_into().unwrap())
                    },
                    length: s.len(),
                },
            },
            Literal::StructInitializer(TemplateInitializer {
                initializer_values,
                named_type,
                ..
            }) => {
                if let Some(ty) = named_type {
                    let ty = self.gen_type(ty.as_ref());

                    let vals: Vec<_> = initializer_values
                        .iter()
                        .map(|f| (f.0.clone(), self.gen_expression(&f.1)))
                        .collect();

                    let field_inits= if let Type::Template { fields, .. } = &ty {
                        let vvv: Option<Vec<(String, Value)>> = vals.into_iter().map(|(name, val)| {

                            let res = fields.get(&name);
                            let mut str_val = None;
                            if let Some(field) = res {
                                let value = val.weak_cast(field, self.builder.get_builder());
                                match value {
                                    Ok(val) => str_val = Some((name, val)),
                                    Err(false) => str_val = Some((name, val)),
                                    Err(true) => {self.add_error(format!(
                                        "Type provided for field `{}` does not match that of template!",
                                        name, 
                                    ));},
                                };
                            } else {
                                self.add_error(format!("Unexpected field initializer `{}`", name))
                            }
                            str_val
                        }).collect();
                        vvv
                    } else {
                        None
                    };

                    if let Some(init) = field_inits {
                        let mut hsh = HashMap::new();
                        for f in init {
                            hsh.insert(f.0, f.1);
                        }
                        Value::TemplateFields {
                            fields: hsh,
                            template_type: ty
                        }
                    //     let llvm_fields: Result<Vec<LLVMValueRef>, CodeGenError> = init.iter().map(|f| f.get_raw_value()).collect();
                    //     let mut llvm_fields = check!(self, llvm_fields, Value);
                    // let llvm_value = unsafe {
                    //     LLVMConstNamedStruct(ty.get_type(), llvm_fields.as_mut_ptr(), llvm_fields.len().try_into().unwrap())
                    // };
                    // let llvm_value = unsafe {
                    //     let glb = LLVM(self.module, ty.get_type(), NULL_STR);
                    //     LLVMSetInitializer(glb, llvm_value);
                    //     glb
                    // };
                    //  Value::Template {
                    //     llvm_value,
                    //     template_type: ty.clone()
                    // }
                    } else {
                        Value::Empty
                    }


                } else {
                    // TODO: Handle idk hash map or something
                    Value::Empty
                }
            }
            _ => {
                self.add_error(String::from("Unsupported literal"));
                Value::Empty
            }
        }
    }
}
