use dsl_errors::check;
use dsl_symbol::{Symbol, SymbolValue};
use linked_hash_map::LinkedHashMap;
use llvm_sys::core::{
    LLVMArrayType, LLVMDoubleType, LLVMFloatType, LLVMFunctionType, LLVMInt1Type, LLVMInt8Type,
    LLVMIntType, LLVMPointerType, LLVMVoidType,
};

use dsl_lexer::ast::{ArrayType, FunctionType, GenericType, ReferenceType, Type};

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
                        constant: false,
                    }
                }
            }
            Type::Unit => unsafe {
                dsl_symbol::Type::Unit {
                    llvm_type: LLVMVoidType(),
                }
            },
            c @ Type::NamedType(token) => {
                let str = token.as_string();

                let sym = self.symbol_root.borrow();
                {
                    let current = self.get_symbol(&sym, &self.current_symbol.borrow());

                    if let Some(current) = current {
                        if let Some(al_sym) = current.children.get(&str) {
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
                    self.find_up_chain(&sym, &self.current_symbol.borrow(), &str)
                {
                    let value = value;
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
            }) => {
                let dsl_symbol::Type::TemplateTemplate { path, fields, ty_params, existing, specialization }= self.gen_type(base_type) else {
                    return dsl_symbol::Type::Empty
                };
                let parameters: Vec<dsl_symbol::Type> =
                    arguments.iter().map(|f| self.gen_type(f)).collect();

                let old_sym = self.current_symbol.replace(path.clone());

                let mut fn_types = vec![];

                {
                    let mut sym = self.symbol_root.borrow_mut();
                    let current = self.get_symbol_mut(&mut sym, &self.current_symbol.borrow());

                    if let Some(current) = current {
                        let mut gens = parameters.iter();

                        let mut pram_iter = ty_params.iter();

                        while let Some(p) = gens.next() {
                            let Some((name, bounds)) = pram_iter.next() else {
                                    self.add_error(format!("Extra generic parameter in template initializer!"));
                                    break;
                                };
                            if let Some(sym) = current.children.get_mut(name) {
                                match &mut sym.value {
                                    SymbolValue::Generic(ty, _) => {
                                        *ty = p.clone();
                                        fn_types.push(p.clone())
                                    }
                                    _ => (),
                                }
                            }
                        }

                        if pram_iter.count() > 0 {
                            self.add_error(format!(
                                "Extra generic parameter in template initializer!"
                            ));
                        }
                    }
                }

                let fn_types_str: Vec<String> = fn_types.iter().map(|f| f.to_string()).collect();

                let mut vars = LinkedHashMap::new();

                let specialization = specialization.iter().find(|(f, _)| {
                    for (a, b) in f.iter().zip(fn_types_str.iter()) {
                        if a == "" {
                            return true;
                        } else if a != b {
                            return false;
                        } else {
                            return true;
                        }
                    }
                    true
                });
                let existing_impl = existing.iter().find(|(f, _)| *f == &fn_types_str);

                let name = if let Some(last) = path.last() {
                    if let Some(ty) = existing_impl {
                        let sym = self.symbol_root.borrow();
                        let current = self.get_symbol(&sym, &ty.1);
                        if let Some(ty) = current {
                            if let SymbolValue::Template(t) = &ty.value {
                                Some((ty.name.clone(), t.clone()))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        let base = path.clone();
                        let path = &path[..path.len() - 1];
                        let name = self.get_next_name(path, last.clone());
                        let (template, name) = {
                            let sym = self.symbol_root.borrow();
                            let opvars = if let Some(special) = specialization {
                                let current = self.get_symbol(&sym, &special.1);

                                if let Some(Symbol {
                                    value:
                                        SymbolValue::Template(dsl_symbol::Type::TemplateTemplate {
                                            fields,
                                            ..
                                        }),
                                    ..
                                }) = current
                                {
                                    Some(fields)
                                } else {
                                    None
                                }
                            } else {
                                None
                            };

                            let mut tyep = Vec::from(path);
                            tyep.push(name.clone());

                            if let Some(opvars) = opvars {
                                for f in opvars.iter() {
                                    let name = f.symbol.as_string();
                                    vars.insert(name.clone(), self.gen_type(&f.symbol_type));
                                }
                            } else {
                                for f in fields.iter() {
                                    let name = f.symbol.as_string();
                                    vars.insert(name.clone(), self.gen_type(&f.symbol_type));
                                }
                            };

                            let mng = self.get_mangled_name_with_path(path, &name);
                            use dsl_symbol::Type;
                            let template = check!(
                                self.errors.borrow_mut(),
                                self.builder.create_struct(
                                    tyep,
                                    Some((base, fn_types.to_vec())),
                                    &mng,
                                    vars.clone()
                                ),
                                Type
                            );

                            (template, name)
                        };

                        let ty = template.clone();
                        self.add_and_set_symbol_from_path(
                            path,
                            &name,
                            SymbolValue::Template(template),
                        );

                        Some((name, ty))
                    }
                } else {
                    None
                };

                self.current_symbol.replace(old_sym);

                if let Some((name, ty)) = name {
                    let mut npath = Vec::from(&path[..path.len() - 1]);
                    npath.push(name);

                    let mut sym = self.symbol_root.borrow_mut();

                    let tmpl_templ = self.get_symbol_mut(&mut sym, &path);
                    if let Some(Symbol {
                        value:
                            SymbolValue::Template(dsl_symbol::Type::TemplateTemplate {
                                existing, ..
                            }),
                        ..
                    }) = tmpl_templ
                    {
                        if existing_impl.is_none() {
                            existing.insert(fn_types_str, npath.clone());
                        }
                    }

                    return ty;
                }

                dsl_symbol::Type::Empty
            }
            c => {
                self.add_error(format!("Unsupported type {:?}", c));
                dsl_symbol::Type::Empty
            }
        }
    }
}
