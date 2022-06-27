use std::collections::HashMap;

use dsl_errors::check;
use dsl_util::{TreeDisplay, NULL_STR};
use linked_hash_map::LinkedHashMap;
use llvm_sys::{
    core::{
        LLVMArrayType, LLVMConstArray, LLVMConstInt, LLVMConstReal, LLVMFloatType, LLVMInt1Type,
        LLVMInt32Type, LLVMInt8Type,
    },
    LLVMCreateStringLiteral,
};

use dsl_lexer::ast::{ArrayInitializer, Literal, TemplateInitializer};

use super::module::Module;
use dsl_symbol::{Symbol, SymbolValue, Type, Value};

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
                    LLVMCreateStringLiteral(
                        self.builder.get_builder(),
                        s.as_ptr() as *const _,
                        s.len().try_into().unwrap(),
                        NULL_STR,
                    )
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

                    if let Type::Generic {
                        base_type:
                            box Type::TemplateTemplate {
                                fields,
                                ty_params,
                                path,
                                existing,
                                specialization,
                            },
                        parameters,
                    } = &ty
                    {
                        let old_sym = self.current_symbol.replace(path.clone());

                        let mut fn_types = vec![];

                        {
                            let mut sym = self.symbol_root.borrow_mut();
                            let current =
                                self.get_symbol_mut(&mut sym, &self.current_symbol.borrow());

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
                                                fn_types.push(ty.to_string())
                                            }
                                            _ => (),
                                        }
                                    }
                                }

                                pram_iter.for_each(|(name, bounds)| {
                                let found = fields.iter().position(|f| {
                                    if let dsl_lexer::ast::Type::NamedType(t) = &f.symbol_type {
                                        if &t.as_string() == name {
                                            return true;
                                        }
                                    }
                                    return false;
                                });
                                if let Some(pos) = found {
                                    if let Some(sym) = current.children.get_mut(name) {
                                        match &mut sym.value {
                                            SymbolValue::Generic(ty, _) => {
                                                *ty = vals[pos].1.get_type().clone();
                                                fn_types.push(ty.to_string())
                                            }
                                            _ => (),
                                        }
                                    }
                                } else {
                                    self.add_error(format!(
                                        "Unable to determine type for generic paramater: `{}` -- Please provide type parameter",
                                        name
                                    ))
                                }
                            })
                            }
                        }

                        let mut vars = LinkedHashMap::new();

                        let specialization = specialization.iter().find(|(f, _)| {
                            for (a, b) in f.iter().zip(fn_types.iter()) {
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
                        let existing_impl = existing.iter().find(|(f, _)| *f == &fn_types);

                        let name = if let Some(last) = path.last() {
                            if let Some(ty) = existing_impl {
                                Some(ty.1.last().unwrap().clone())
                            } else {
                                let path = &path[..path.len() - 1];
                                let name = self.get_next_name(path, last.clone());
                                let (template, name) = {
                                    let sym = self.symbol_root.borrow();
                                    let opvars = if let Some(special) = specialization {
                                        let current = self.get_symbol(&sym, &special.1);

                                        if let Some(Symbol {
                                            value:
                                                SymbolValue::Template(Type::TemplateTemplate {
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
                                            vars.insert(
                                                name.clone(),
                                                self.gen_type(&f.symbol_type),
                                            );
                                        }
                                    } else {
                                        for f in fields.iter() {
                                            let name = f.symbol.as_string();
                                            vars.insert(
                                                name.clone(),
                                                self.gen_type(&f.symbol_type),
                                            );
                                        }
                                    };

                                    let mng = self.get_mangled_name_with_path(path, &name);
                                    let template = check!(
                                        self.errors.borrow_mut(),
                                        self.builder.create_struct(&path, &mng, vars.clone()),
                                        Value
                                    );

                                    (template, name)
                                };

                                self.add_and_set_symbol_from_path(
                                    path,
                                    &name,
                                    SymbolValue::Template(template),
                                );

                                Some(name)
                            }
                        } else {
                            None
                        };

                        self.current_symbol.replace(old_sym);

                        if let Some(name) = name {
                            let mut npath = Vec::from(&path[..path.len() - 1]);
                            npath.push(name);

                            let mut sym = self.symbol_root.borrow_mut();

                            let tmpl_templ = self.get_symbol_mut(&mut sym, &path);
                            if let Some(Symbol {
                                value:
                                    SymbolValue::Template(Type::TemplateTemplate { existing, .. }),
                                ..
                            }) = tmpl_templ
                            {
                                if existing_impl.is_none() {
                                    existing.insert(fn_types, npath.clone());
                                }
                            }

                            let current = self.get_symbol(&mut sym, &npath);
                            if let Some(Symbol {
                                value: SymbolValue::Template(val @ Type::Template { fields, .. }),
                                ..
                            }) = current
                            {
                                let vvv: Option<Vec<(String, Value)>> = vals.into_iter().map(|(name, val)| {

                                    let res = if existing_impl.is_none() {
                                        vars.get(&name)
                                    } else {
                                        fields.get(&name)
                                    };

                                    let mut str_val = None;
                                    if let Some(field) = res {
                                        let value = val.weak_cast(field, self.builder.get_builder());
                                        match value {
                                            Ok(val) => str_val = Some((name, val)),
                                            Err(false) => str_val = Some((name, val)),
                                            Err(true) => {self.add_error(format!(
                                                "Type provided for field `{}` does not match that of template!",
                                                name,
                                            ));
                                        },
                                        };
                                    } else {
                                        self.add_error(format!("Unexpected field initializer `{}` (1)", name))
                                    }
                                    str_val
                                }).collect();

                                if let Some(init) = vvv {
                                    let mut hsh = HashMap::new();
                                    for f in init {
                                        hsh.insert(f.0, f.1);
                                    }
                                    return Value::TemplateFields {
                                        fields: hsh,
                                        template_type: val.clone(),
                                    };
                                }
                            }
                        }

                        Value::Empty
                    } else if let Type::TemplateTemplate {
                        fields,
                        ty_params,
                        path,
                        existing,
                        specialization,
                    } = &ty
                    {
                        let old_sym = self.current_symbol.replace(path.clone());

                        let mut fn_types = vec![];

                        {
                            let mut sym = self.symbol_root.borrow_mut();
                            let current =
                                self.get_symbol_mut(&mut sym, &self.current_symbol.borrow());

                            if let Some(current) = current {
                                ty_params.iter().for_each(|(name, bounds)| {
                                    let found = fields.iter().position(|f| {
                                        if let dsl_lexer::ast::Type::NamedType(t) = &f.symbol_type {
                                            if &t.as_string() == name {
                                                return true;
                                            }
                                        }

                                        return false;
                                    });
                                    if let Some(pos) = found {
                                        if let Some(sym) = current.children.get_mut(name) {
                                            match &mut sym.value {
                                                SymbolValue::Generic(ty, _) => {
                                                    *ty = vals[pos].1.get_type().clone();
                                                    fn_types.push(ty.to_string())
                                                }
                                                _ => (),
                                            }
                                        }
                                    } else {
                                        self.add_error(format!(
                                            "Unable to determine type for generic paramater: `{}` -- Please provide type parameter",
                                            name
                                        ))
                                    }
                                })
                            }
                        }

                        let specialization = specialization.iter().find(|(f, _)| {
                            for (a, b) in f.iter().zip(fn_types.iter()) {
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
                        let existing_impl = existing.iter().find(|(f, _)| *f == &fn_types);

                        let mut vars = LinkedHashMap::new();

                        let name = if let Some(last) = path.last() {
                            if let Some(ty) = existing_impl {
                                Some(ty.1.last().unwrap().clone())
                            } else {
                                let path = &path[..path.len() - 1];
                                let name = self.get_next_name(path, last.clone());

                                let (template, name) = {
                                    let sym = self.symbol_root.borrow();
                                    let opvars = if let Some(special) = specialization {
                                        let current = self.get_symbol(&sym, &special.1);

                                        if let Some(Symbol {
                                            value:
                                                SymbolValue::Template(Type::TemplateTemplate {
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
                                            vars.insert(
                                                name.clone(),
                                                self.gen_type(&f.symbol_type),
                                            );
                                        }
                                    } else {
                                        for f in fields.iter() {
                                            let name = f.symbol.as_string();
                                            vars.insert(
                                                name.clone(),
                                                self.gen_type(&f.symbol_type),
                                            );
                                        }
                                    };

                                    let mng = self.get_mangled_name_with_path(path, &name);
                                    let template = check!(
                                        self.errors.borrow_mut(),
                                        self.builder.create_struct(&path, &mng, vars.clone()),
                                        Value
                                    );
                                    (template, name)
                                };

                                self.add_and_set_symbol_from_path(
                                    path,
                                    &name,
                                    SymbolValue::Template(template),
                                );

                                Some(name)
                            }
                        } else {
                            None
                        };

                        self.current_symbol.replace(old_sym);

                        if let Some(name) = name {
                            let mut npath = Vec::from(&path[..path.len() - 1]);
                            npath.push(name);

                            let mut sym = self.symbol_root.borrow_mut();

                            let tmpl_templ = self.get_symbol_mut(&mut sym, &path);
                            if let Some(Symbol {
                                value:
                                    SymbolValue::Template(Type::TemplateTemplate { existing, .. }),
                                ..
                            }) = tmpl_templ
                            {
                                if existing_impl.is_none() {
                                    existing.insert(fn_types, npath.clone());
                                }
                            }
                            let current = self.get_symbol(&mut sym, &npath);
                            if let Some(Symbol {
                                value: SymbolValue::Template(val @ Type::Template { fields, .. }),
                                ..
                            }) = current
                            {
                                let vvv: Option<Vec<(String, Value)>> = vals.into_iter().map(|(name, val)| {

                                    let res = if existing_impl.is_none() {
                                        vars.get(&name)
                                    } else {
                                        fields.get(&name)
                                    };

                                    let mut str_val = None;
                                    if let Some(field) = res {
                                        let value = val.weak_cast(field, self.builder.get_builder());
                                        match value {
                                            Ok(val) => str_val = Some((name, val)),
                                            Err(false) => str_val = Some((name, val)),
                                            Err(true) => {self.add_error(format!(
                                                "Type provided for field `{}` does not match that of template!",
                                                name,
                                            ));
                                        },
                                        };
                                    } else {
                                        self.add_error(format!("Unexpected field initializer `{}` (2)", name))
                                    }
                                    str_val
                                }).collect();

                                if let Some(init) = vvv {
                                    let mut hsh = HashMap::new();
                                    for f in init {
                                        hsh.insert(f.0, f.1);
                                    }
                                    return Value::TemplateFields {
                                        fields: hsh,
                                        template_type: val.clone(),
                                    };
                                }
                            }
                        }

                        Value::Empty
                    } else {
                        let field_inits = if let Type::Template { fields, .. } = &ty {
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
                                self.add_error(format!("Unexpected field initializer `{}` (3)", name))
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
                                template_type: ty,
                            }
                        } else {
                            Value::Empty
                        }
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
