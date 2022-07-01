#![feature(box_patterns)]
use std::{collections::HashMap, ffi::CString, fmt::Display};

use dsl_errors::CodeGenError;
use dsl_lexer::ast::{Expression, FunctionSignature, ParseNode, TypeSymbol};
use dsl_util::{CreateParent, Grouper, TreeDisplay, NULL_STR};
use linked_hash_map::LinkedHashMap;
use llvm_sys::{
    core::{
        LLVMBuildBitCast, LLVMBuildIntCast2, LLVMBuildLoad2, LLVMBuildSExt, LLVMBuildZExt,
        LLVMConstArray, LLVMGetAlignment, LLVMGetArrayLength, LLVMGetIntTypeWidth, LLVMGetTypeKind,
        LLVMInt8Type, LLVMPointerType, LLVMPrintTypeToString, LLVMPrintValueToString, LLVMVoidType,
    },
    prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef},
    target::{LLVMGetModuleDataLayout, LLVMPreferredAlignmentOfType},
    LLVMBuildAlignedLoad, LLVMCreateGlobalValue, LLVMGetTypeSize, LLVMGetValueAt, LLVMTypeKind,
};

use bitflags::bitflags;

#[derive(Debug, Clone)]
pub enum GenericType {
    Generic(Option<Vec<Type>>),
    Specialization(Type),
}

impl ToString for GenericType {
    fn to_string(&self) -> String {
        match self {
            GenericType::Generic(_) => "".to_string(),
            GenericType::Specialization(sp) => sp.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Empty,
    Literal {
        llvm_value: LLVMValueRef,
        literal_type: Type,
    },
    Variable {
        llvm_value: LLVMValueRef,
        variable_type: Type,
        constant: bool,
    },
    Function {
        llvm_value: LLVMValueRef,
        function_type: Type,
    },
    MemberFunction {
        func: Box<Value>,
        var: Box<Value>,
    },
    FunctionTemplate {
        path: Vec<String>,
        ty: FunctionSignature,
        body: Box<ParseNode>,
        ty_params: LinkedHashMap<String, GenericType>,
        existing: HashMap<Vec<String>, Vec<String>>,
        specialization: HashMap<Vec<String>, Vec<String>>,
    },
    SpecFunction {
        parameters: Vec<(String, Type)>,
        return_type: Type,
    },
    Template {
        llvm_value: LLVMValueRef,
        template_type: Type,
    },
    TemplateFields {
        fields: HashMap<String, Value>,
        template_type: Type,
    },
    Instruction {
        llvm_value: LLVMValueRef,
    },
    Block {
        llvm_value: LLVMBasicBlockRef,
    },
    Load {
        llvm_value: LLVMValueRef,
        load_type: Type,
        constant: bool,
    },
    Path {
        path: Vec<String>,
    },
}

impl Default for Value {
    fn default() -> Self {
        Self::Empty
    }
}

impl Value {
    pub fn get_type(&self) -> &Type {
        match self {
            Self::Literal { literal_type, .. } => literal_type,
            Self::Variable { variable_type, .. } => variable_type,
            Self::Function { function_type, .. } => function_type,
            Self::MemberFunction { func, .. } => func.get_type(),
            Self::Template { template_type, .. } => template_type,
            Self::TemplateFields { template_type, .. } => template_type,
            Self::Load { load_type, .. } => load_type,
            _ => {
                panic!("Called on unkown value!")
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Value::Empty => true,
            _ => false,
        }
    }

    pub fn is_const(&self) -> bool {
        match self {
            Value::Variable {
                constant,
                variable_type,
                ..
            } => *constant | variable_type.is_const(),
            _ => panic!(),
        }
    }

    pub fn has_value(&self) -> bool {
        match self {
            Value::Empty | Value::Instruction { .. } | Value::Block { .. } => false,
            _ => true,
        }
    }

    pub fn get_value(
        &self,
        builder: LLVMBuilderRef,
        module: LLVMModuleRef,
    ) -> Result<LLVMValueRef, CodeGenError> {
        match self {
            Value::Function { llvm_value, .. } => Ok(*llvm_value),
            Value::MemberFunction { func, .. } => func.get_value(builder, module),
            Value::Template { llvm_value, .. } => Ok(*llvm_value),
            Value::Variable {
                llvm_value,
                variable_type,
                ..
            } => unsafe {
                let mods = LLVMGetModuleDataLayout(module);
                let align = LLVMPreferredAlignmentOfType(mods, variable_type.get_type());
                Ok(LLVMBuildAlignedLoad(
                    builder,
                    variable_type.get_type(),
                    *llvm_value,
                    align,
                    NULL_STR,
                ))
            },
            Value::Literal { llvm_value, .. } => Ok(*llvm_value),
            Value::Load { llvm_value, .. } => Ok(*llvm_value),
            Value::Instruction { llvm_value, .. } => Ok(*llvm_value),
            _ => Err(CodeGenError {
                message: "Value didn't contain any usable data!".into(),
            }),
        }
    }

    pub fn get_raw_value(&self) -> Result<LLVMValueRef, CodeGenError> {
        match self {
            Value::Function { llvm_value, .. } => Ok(*llvm_value),
            Value::MemberFunction { func, .. } => func.get_raw_value(),
            Value::Template { llvm_value, .. } => Ok(*llvm_value),
            Value::Variable { llvm_value, .. } => Ok(*llvm_value),
            Value::Literal { llvm_value, .. } => Ok(*llvm_value),
            Value::Load { llvm_value, .. } => Ok(*llvm_value),

            Value::Instruction { llvm_value, .. } => Ok(*llvm_value),
            _ => Err(CodeGenError {
                message: "Value didn't contain any usable data!".into(),
            }),
        }
    }

    pub fn get_alignment(&self) -> Result<u32, CodeGenError> {
        let val = self.get_raw_value()?;
        let align = unsafe { LLVMGetAlignment(val) };
        Ok(align)
    }

    pub fn get_size(&self, module: LLVMModuleRef) -> Result<u64, CodeGenError> {
        let ty = self.get_type().get_type();
        let size = unsafe { LLVMGetTypeSize(module, ty) };
        Ok(size)
    }

    pub fn weak_cast(&self, to_type: &Type, builder: LLVMBuilderRef) -> Result<Value, bool> {
        if self.get_type() == to_type {
            return Err(false);
        }
        match (self, to_type) {
            (
                Value::Variable {
                    llvm_value: lvalue,
                    variable_type:
                        Type::Array {
                            base_type: lbtype, ..
                        },
                    ..
                }
                | Value::Literal {
                    llvm_value: lvalue,
                    literal_type:
                        Type::Array {
                            base_type: lbtype, ..
                        },
                },
                Type::Reference {
                    llvm_type: rtype,
                    base_type: rbtype,
                    ..
                },
            ) => {
                if lbtype != rbtype {
                    return Err(true);
                }
                let val = unsafe { LLVMBuildBitCast(builder, *lvalue, *rtype, NULL_STR) };

                return Ok(Value::Literal {
                    llvm_value: val,
                    literal_type: to_type.clone(),
                });
            }
            (
                Value::Variable {
                    llvm_value: lvalue,
                    variable_type: Type::String { .. },
                    ..
                }
                | Value::Literal {
                    llvm_value: lvalue,
                    literal_type: Type::String { .. },
                },
                Type::Reference {
                    llvm_type: rtype,
                    base_type: rbtype,
                    ..
                },
            ) => {
                if (Type::Integer {
                    llvm_type: unsafe { LLVMInt8Type() },
                    signed: false,
                }) != **rbtype
                {
                    return Err(true);
                }
                let val = unsafe { LLVMBuildBitCast(builder, *lvalue, *rtype, NULL_STR) };

                return Ok(Value::Literal {
                    llvm_value: val,
                    literal_type: to_type.clone(),
                });
            }
            (
                Value::Literal {
                    llvm_value,
                    literal_type:
                        Type::Integer {
                            llvm_type: ltype,
                            signed: lsigned,
                        },
                },
                Type::Integer {
                    llvm_type: rtype,
                    signed: rsigned,
                },
            ) => {
                if *ltype != *rtype {
                    return Ok(Value::Literal {
                        llvm_value: unsafe {
                            LLVMBuildIntCast2(
                                builder,
                                *llvm_value,
                                *rtype,
                                if *rsigned { 1 } else { 0 },
                                NULL_STR,
                            )
                        },
                        literal_type: to_type.clone(),
                    });
                } else if *lsigned && !rsigned {
                    return Ok(Value::Literal {
                        llvm_value: unsafe {
                            LLVMBuildZExt(builder, *llvm_value, *rtype, NULL_STR)
                        },
                        literal_type: to_type.clone(),
                    });
                } else if !lsigned && *rsigned {
                    return Ok(Value::Literal {
                        llvm_value: unsafe {
                            LLVMBuildSExt(builder, *llvm_value, *rtype, NULL_STR)
                        },
                        literal_type: to_type.clone(),
                    });
                }
            }
            (
                Value::Literal {
                    llvm_value,
                    literal_type:
                        Type::Array {
                            base_type: lbtype,
                            llvm_type: ltype,
                        },
                },
                Type::Array {
                    base_type: rbtype,
                    llvm_type: rtype,
                },
            ) => {
                let llen = unsafe { LLVMGetArrayLength(*ltype) };
                let rlen = unsafe { LLVMGetArrayLength(*rtype) };
                if llen != rlen {
                    return Err(true);
                }

                let res: Result<Vec<Value>, bool> = (0..llen)
                    .into_iter()
                    .map(|i| {
                        let value = unsafe { LLVMGetValueAt(*llvm_value, i) };

                        Value::Literal {
                            llvm_value: value,
                            literal_type: *lbtype.clone(),
                        }
                        .weak_cast(&rbtype, builder)
                    })
                    .collect();
                let res = res?;

                let r: Result<Vec<LLVMValueRef>, CodeGenError> =
                    res.iter().map(|f| f.get_raw_value()).collect();
                let mut r = match r {
                    Ok(r) => r,
                    Err(_) => return Err(true),
                };

                let llvm_value =
                    unsafe { LLVMConstArray(*rtype, r.as_mut_ptr(), r.len().try_into().unwrap()) };

                return Ok(Value::Literal {
                    llvm_value,
                    literal_type: to_type.clone(),
                });
            }
            (
                Value::Literal {
                    llvm_value,
                    literal_type: Type::Reference { base_type, .. },
                },
                Type::Reference { base_type: rty, .. },
            ) => {
                return Value::Literal {
                    llvm_value: *llvm_value,
                    literal_type: *base_type.clone(),
                }
                .weak_cast(&rty, builder);
            }
            _ => (),
        }

        Err(true)
    }

    pub fn get_ptr(&self, module: LLVMModuleRef) -> Value {
        match self {
            Value::Variable {
                llvm_value,
                variable_type,
                constant,
            } => Value::Literal {
                llvm_value: *llvm_value,
                literal_type: variable_type.get_ptr(*constant),
            },
            Value::Literal {
                literal_type,
                llvm_value,
            } => Value::Literal {
                llvm_value: unsafe {
                    LLVMCreateGlobalValue(module, literal_type.get_type(), *llvm_value)
                },
                literal_type: literal_type.get_ptr(true),
            },
            _ => panic!(),
        }
    }

    pub fn llvm_string(&self) -> String {
        let cstr = unsafe { LLVMPrintValueToString(self.get_raw_value().unwrap()) };
        let s = unsafe { CString::from_raw(cstr as *mut _) };
        s.into_string().unwrap()
    }

    pub fn resolve_base_value(&self, builder: LLVMBuilderRef) -> Value {
        match self {
            Value::Variable {
                llvm_value,
                variable_type:
                    Type::Reference {
                        base_type,
                        llvm_type,
                        constant,
                        ..
                    },
                ..
            } => unsafe {
                let llvm_value = LLVMBuildLoad2(builder, *llvm_type, *llvm_value, NULL_STR);
                let ret = Value::Load {
                    llvm_value,
                    load_type: *base_type.clone(),
                    constant: *constant,
                };
                ret
            },
            _ => self.clone(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Literal { literal_type, .. } => write!(f, "{}", literal_type),
            Value::Variable { variable_type, .. } => write!(f, "{}", variable_type),
            Value::Function { function_type, .. } => write!(f, "{}", function_type),
            Value::SpecFunction { .. } => write!(f, ""),
            Value::MemberFunction { func, .. } => write!(f, "{}", func),
            Value::Template { template_type, .. } => write!(f, "{}", template_type),
            Value::TemplateFields { template_type, .. } => write!(f, "{}", template_type),
            Value::Load { load_type, .. } => write!(f, "{}", load_type),
            Value::Instruction { .. } => write!(f, "Instruction"),
            Value::Block { .. } => write!(f, "Block"),
            Value::FunctionTemplate { .. } => write!(f, "Function Template"),
            Value::Empty => write!(f, "Empty"),
            Value::Path { path } => write!(f, "Path {:?}", path),
        }
    }
}

impl TreeDisplay for Value {
    fn num_children(&self) -> usize {
        match self {
            Value::Literal { literal_type, .. } => literal_type.num_children(),
            Value::Variable { variable_type, .. } => variable_type.num_children(),
            Value::Function { function_type, .. } => function_type.num_children(),
            Value::Template { template_type, .. } => template_type.num_children(),
            Value::Load { load_type, .. } => load_type.num_children(),
            _ => 0,
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match self {
            Value::Literal { literal_type, .. } => literal_type.child_at(index),
            Value::Variable { variable_type, .. } => variable_type.child_at(index),
            Value::Function { function_type, .. } => function_type.child_at(index),
            Value::Template { template_type, .. } => template_type.child_at(index),
            Value::Load { load_type, .. } => load_type.child_at(index),
            _ => panic!(),
        }
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<dyn TreeDisplay + 'a> {
        match self {
            Value::Function { function_type, .. } => function_type.child_at_bx(index),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Empty,
    Unit {
        llvm_type: LLVMTypeRef,
    },
    Integer {
        llvm_type: LLVMTypeRef,
        signed: bool,
    },
    Float {
        llvm_type: LLVMTypeRef,
    },
    Boolean {
        llvm_type: LLVMTypeRef,
    },
    Char {
        llvm_type: LLVMTypeRef,
    },
    String {
        llvm_type: LLVMTypeRef,
        length: usize,
    },
    Array {
        llvm_type: LLVMTypeRef,
        base_type: Box<Type>,
    },
    Reference {
        llvm_type: LLVMTypeRef,
        base_type: Box<Type>,
        constant: bool,
    },
    Function {
        llvm_type: LLVMTypeRef,
        parameters: LinkedHashMap<String, Type>,
        return_type: Box<Type>,
    },
    Template {
        llvm_type: LLVMTypeRef,
        fields: LinkedHashMap<String, Type>,
        path: Vec<String>,
        base_template: Option<(Vec<String>, Vec<Type>)>,
    },
    TemplateTemplate {
        // llvm_type: LLVMTypeRef,
        path: Vec<String>,
        fields: Vec<TypeSymbol>,
        // path: Vec<String>,
        ty_params: LinkedHashMap<String, GenericType>,

        existing: HashMap<Vec<String>, Vec<String>>,
        specialization: HashMap<Vec<String>, Vec<String>>,
    },
    Generic {
        base_type: Box<Type>,
        parameters: Vec<Type>,
    },
    Spec {
        path: Vec<String>,
    },
}

impl Type {
    pub fn is_empty(&self) -> bool {
        match self {
            Type::Empty => true,
            _ => false,
        }
    }

    pub fn get_type(&self) -> LLVMTypeRef {
        match self {
            Self::Integer { llvm_type, .. } => *llvm_type,
            Self::Float { llvm_type, .. } => *llvm_type,
            Self::Boolean { llvm_type, .. } => *llvm_type,
            Self::Char { llvm_type, .. } => *llvm_type,
            Self::String { llvm_type, .. } => *llvm_type,
            Self::Array { llvm_type, .. } => *llvm_type,
            Self::Unit { llvm_type, .. } => *llvm_type,
            Self::Reference { llvm_type, .. } => *llvm_type,
            Self::Function { llvm_type, .. } => unsafe { LLVMPointerType(*llvm_type, 0) },
            Self::Template { llvm_type, .. } => *llvm_type,
            _ => {
                panic!("Called on unkown value!")
            }
        }
    }

    pub fn unit_ty() -> Type {
        Type::Unit {
            llvm_type: unsafe { LLVMVoidType() },
        }
    }

    pub fn resolve_properties<'a>(&'a self) -> Option<&'a LinkedHashMap<String, Type>> {
        match self {
            Type::Template { fields, .. } => Some(fields),
            _ => None,
        }
    }

    pub fn resolve_base_type(&self) -> &Type {
        match self {
            Type::Reference { base_type, .. } => base_type.resolve_base_type(),
            _ => self,
        }
    }

    pub fn resolve_path(&self) -> Option<Vec<String>> {
        match self {
            Type::Template { path, .. } => Some(path.clone()),
            Type::TemplateTemplate { path, .. } => Some(path.clone()),
            Type::Integer {
                signed: true,
                llvm_type,
            } => {
                let w = unsafe { LLVMGetIntTypeWidth(*llvm_type) };
                Some(vec!["core".to_string(), format!("int{}", w)])
            }
            Type::Integer {
                signed: false,
                llvm_type,
            } => {
                let w = unsafe { LLVMGetIntTypeWidth(*llvm_type) };
                Some(vec!["core".to_string(), format!("uint{}", w)])
            }
            Type::Boolean { .. } => Some(vec!["core".to_string(), format!("bool")]),
            Type::Float { llvm_type } => {
                let kind = unsafe { LLVMGetTypeKind(*llvm_type) };
                match kind {
                    LLVMTypeKind::LLVMFloatTypeKind => {
                        Some(vec!["core".to_string(), format!("float32")])
                    }
                    LLVMTypeKind::LLVMDoubleTypeKind => {
                        Some(vec!["core".to_string(), format!("float64")])
                    }
                    _ => None,
                }
            }
            Type::Char { .. } => Some(vec!["core".to_string(), format!("char")]),
            Type::Generic { base_type, .. } => base_type.resolve_path(),
            _ => None,
        }
    }

    pub fn get_ptr(&self, constant: bool) -> Type {
        let ty = self.get_type();
        let llvm_type = unsafe { LLVMPointerType(ty, 0) };
        Type::Reference {
            llvm_type,
            base_type: Box::new(self.clone()),
            constant,
        }
    }

    pub fn is_const(&self) -> bool {
        match self {
            Type::Reference { constant, .. } => *constant,
            _ => false,
        }
    }

    pub fn llvm_string(&self) -> String {
        let cstr = unsafe { LLVMPrintTypeToString(self.get_type()) };
        let s = unsafe { CString::from_raw(cstr as *mut _) };
        s.into_string().unwrap()
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, ""),
            Self::Unit { .. } => f.write_str("()"),
            Self::Integer {
                signed: true,
                llvm_type,
            } => {
                let width = unsafe { LLVMGetIntTypeWidth(*llvm_type) };
                write!(f, "int{}", width)
            }
            Self::Integer {
                signed: false,
                llvm_type,
            } => {
                let width = unsafe { LLVMGetIntTypeWidth(*llvm_type) };
                write!(f, "uint{}", width)
            }
            Self::Boolean { .. } => write!(f, "bool"),
            Self::Float { llvm_type } => {
                let ty = unsafe { LLVMGetTypeKind(*llvm_type) };
                match ty {
                    llvm_sys::LLVMTypeKind::LLVMFloatTypeKind => write!(f, "float32"),
                    llvm_sys::LLVMTypeKind::LLVMDoubleTypeKind => write!(f, "float64"),
                    _ => write!(f, ""),
                }
            }
            Self::Char { .. } => write!(f, "char"),
            Self::Array { base_type, .. } => {
                write!(f, "[{}]", base_type)
            }
            Self::Function {
                parameters,
                return_type,
                ..
            } => {
                write!(f, "(")?;
                if parameters.len() >= 1 {
                    let mut iter = parameters.iter();
                    write!(f, "{}", iter.next().unwrap().1)?;
                    for (_, t) in iter {
                        write!(f, ", {}", t)?;
                    }
                }
                write!(f, ")")?;
                if let Type::Unit { .. } = **return_type {
                    write!(f, " =>")
                } else {
                    write!(f, ":{} =>", return_type)
                }
            }
            Self::Reference { base_type, .. } => {
                write!(f, "&{}", base_type)
            }
            Self::String { .. } => {
                write!(f, "string")
            }
            Self::Template { .. } => {
                write!(f, "**Template**")
            }
            Self::Spec { .. } => {
                write!(f, "**Spec**")
            }
            Self::TemplateTemplate { path, .. } => {
                if let Some(p) = path.last() {
                    write!(f, "{}", p)
                } else {
                    write!(f, "**Template**")
                }
            }
            Self::Generic { .. } => {
                write!(f, "**Generic**")
            } // Self::(GenericType {
              //     arguments,
              //     base_type,
              //     ..
              // }) => {
              //     write!(f, "{}<", base_type)?;
              //     write!(f, "{}", arguments[0])?;
              //     for t in &arguments[1..] {
              //         write!(f, ", {}", t)?;
              //     }
              //     write!(f, ">")
              // }
        }
    }
}

impl TreeDisplay for Type {
    fn num_children(&self) -> usize {
        match self {
            Type::Array { .. } => 1,
            Type::Reference { .. } => 1,
            Type::Function { parameters, .. } => parameters.len() + 1,
            Type::Template { fields, .. } => fields.len(),
            _ => 0,
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match self {
            Type::Array { base_type, .. } => Some(base_type.as_ref()),
            Type::Reference { base_type, .. } => Some(base_type.as_ref()),
            Type::Function { return_type, .. } if index == 0 => Some(return_type.as_ref()),
            Type::Function { .. } => None,
            Type::Template { .. } => None,
            _ => panic!(),
        }
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<dyn TreeDisplay + 'a> {
        match self {
            Type::Function { parameters, .. } => Box::new(CreateParent(
                parameters.keys().nth(index - 1).unwrap().clone(),
                vec![parameters.values().nth(index - 1).unwrap()],
            )),
            Type::Template { fields, .. } => Box::new(Grouper(format!(
                "{}: {}",
                fields.keys().nth(index).unwrap(),
                fields.values().nth(index).unwrap()
            ))),
            _ => panic!(),
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Unit {
                    llvm_type: l_llvm_type,
                },
                Self::Unit {
                    llvm_type: r_llvm_type,
                },
            ) => l_llvm_type == r_llvm_type,
            (
                Self::Integer {
                    llvm_type: l_llvm_type,
                    signed: l_signed,
                },
                Self::Integer {
                    llvm_type: r_llvm_type,
                    signed: r_signed,
                },
            ) => l_llvm_type == r_llvm_type && l_signed == r_signed,
            (
                Self::Float {
                    llvm_type: l_llvm_type,
                },
                Self::Float {
                    llvm_type: r_llvm_type,
                },
            ) => l_llvm_type == r_llvm_type,
            (
                Self::Boolean {
                    llvm_type: l_llvm_type,
                },
                Self::Boolean {
                    llvm_type: r_llvm_type,
                },
            ) => l_llvm_type == r_llvm_type,
            (
                Self::Char {
                    llvm_type: l_llvm_type,
                },
                Self::Char {
                    llvm_type: r_llvm_type,
                },
            ) => l_llvm_type == r_llvm_type,
            (
                Self::String {
                    llvm_type: l_llvm_type,
                    length: l_length,
                },
                Self::String {
                    llvm_type: r_llvm_type,
                    length: r_length,
                },
            ) => l_llvm_type == r_llvm_type && l_length == r_length,
            (
                Self::Array {
                    llvm_type: l_llvm_type,
                    base_type: l_base_type,
                },
                Self::Array {
                    llvm_type: r_llvm_type,
                    base_type: r_base_type,
                },
            ) => l_llvm_type == r_llvm_type && l_base_type == r_base_type,
            (
                Self::Reference {
                    llvm_type: l_llvm_type,
                    base_type: l_base_type,
                    constant: l_const,
                },
                Self::Reference {
                    llvm_type: r_llvm_type,
                    base_type: r_base_type,
                    constant: r_const,
                },
            ) => l_llvm_type == r_llvm_type && l_base_type == r_base_type && l_const == r_const,
            (
                Self::Function {
                    parameters: l_parameters,
                    return_type: l_return_type,
                    ..
                },
                Self::Function {
                    parameters: r_parameters,
                    return_type: r_return_type,
                    ..
                },
            ) => {
                for p in r_parameters.values().zip(l_parameters.values()) {
                    if p.0 != p.1 {
                        return false;
                    }
                }

                if l_return_type != r_return_type {
                    return false;
                }

                true
            }
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

bitflags! {
    pub struct SymbolFlags: u32 {
        const CONSTANT = 0b00000001;
        const EXPORT = 0b00000010;
    }
}

pub enum SymbolValue {
    Empty,
    Variable(Value),
    Function(Value),
    // SpecFunction(Value),
    Field(Type),
    Template(Type),
    Action(Type),
    Spec(Type),
    Alias(Type),
    Generic(Type, GenericType),
    Primitive(Type),
    Macro(Box<dyn Fn(&Vec<Expression>) -> ParseNode>),
    Module,
}

impl std::fmt::Debug for SymbolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty"),
            Self::Variable(arg0) => f.debug_tuple("Variable").field(arg0).finish(),
            Self::Function(arg0) => f.debug_tuple("Function").field(arg0).finish(),
            Self::Field(arg0) => f.debug_tuple("Field").field(arg0).finish(),
            Self::Template(arg0) => f.debug_tuple("Template").field(arg0).finish(),
            Self::Action(arg0) => f.debug_tuple("Action").field(arg0).finish(),
            Self::Spec(arg0) => f.debug_tuple("Spec").field(arg0).finish(),
            Self::Alias(arg0) => f.debug_tuple("Alias").field(arg0).finish(),
            Self::Generic(arg0, arg1) => f.debug_tuple("Generic").field(arg0).field(arg1).finish(),
            Self::Primitive(arg0) => f.debug_tuple("Primitive").field(arg0).finish(),
            Self::Macro(arg0) => f.debug_tuple("Macro").finish(),
            Self::Module => write!(f, "Module"),
        }
    }
}

impl SymbolValue {
    pub fn is_const(&self) -> bool {
        match self {
            Self::Variable(v) => v.is_const(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub value: SymbolValue,
    pub children: LinkedHashMap<String, Symbol>,
    pub flags: SymbolFlags,
}

impl Symbol {
    pub fn root() -> Symbol {
        Symbol {
            name: String::from("root"),
            value: SymbolValue::Empty,
            children: LinkedHashMap::new(),
            flags: SymbolFlags::empty(),
        }
    }

    pub fn new(name: String, value: SymbolValue) -> Symbol {
        Symbol {
            name,
            value,
            children: LinkedHashMap::new(),
            flags: SymbolFlags::empty(),
        }
    }

    pub fn new_flags(name: String, value: SymbolValue, flags: SymbolFlags) -> Symbol {
        Symbol {
            name,
            value,
            children: LinkedHashMap::new(),
            flags,
        }
    }

    pub fn add_child<T: ToString>(&mut self, name: &T, value: SymbolValue) -> &mut Symbol {
        self.children
            .insert(name.to_string(), Symbol::new(name.to_string(), value));
        self.children.get_mut(&name.to_string()).unwrap()
    }

    pub fn add_child_flags<T: ToString>(
        &mut self,
        name: &T,
        value: SymbolValue,
        flags: SymbolFlags,
    ) -> &mut Symbol {
        self.children.insert(
            name.to_string(),
            Symbol::new_flags(name.to_string(), value, flags),
        );
        self.children.get_mut(&name.to_string()).unwrap()
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.flags.is_empty() {
            match self.value {
                SymbolValue::Variable(_) => write!(f, "Variable `{}`", self.name),
                SymbolValue::Action(_) => write!(f, "Action `{}`", self.name),
                SymbolValue::Alias(_) => write!(f, "Alias `{}`", self.name),
                SymbolValue::Empty => write!(f, "{}", self.name),
                SymbolValue::Field(_) => write!(f, "Field `{}`", self.name),
                SymbolValue::Macro(_) => write!(f, "Maro `{}`", self.name),
                SymbolValue::Function(_) => write!(f, "Function `{}`", self.name),
                // SymbolValue::SpecFunction(_) => write!(f, "Spec Function `{}`", self.name),
                SymbolValue::Module => write!(f, "Module `{}`", self.name),
                SymbolValue::Spec(_) => write!(f, "Spec `{}`", self.name),
                SymbolValue::Template(_) => write!(f, "Template `{}`", self.name),
                SymbolValue::Primitive(_) => write!(f, "Primititve `{}`", self.name),
                SymbolValue::Generic(_, _) => write!(f, "Generic `{}`", self.name),
            }
        } else {
            match self.value {
                SymbolValue::Variable(_) => {
                    write!(f, "Variable `{}` ({:?})", self.name, self.flags)
                }
                SymbolValue::Action(_) => write!(f, "Action `{}` ({:?})", self.name, self.flags),
                SymbolValue::Alias(_) => write!(f, "Alias `{}` ({:?})", self.name, self.flags),
                SymbolValue::Empty => write!(f, "{} ({:?})", self.name, self.flags),
                SymbolValue::Macro(_) => write!(f, "Macro {} ({:?})", self.name, self.flags),
                SymbolValue::Field(_) => write!(f, "Field `{}` ({:?})", self.name, self.flags),
                SymbolValue::Function(_) => {
                    write!(f, "Function `{}` ({:?})", self.name, self.flags)
                }
                // SymbolValue::SpecFunction(_) => {
                // write!(f, "Spec Function `{}` ({:?})", self.name, self.flags)
                // }
                SymbolValue::Module => write!(f, "Module `{}` ({:?})", self.name, self.flags),
                SymbolValue::Spec(_) => write!(f, "Spec `{}` ({:?})", self.name, self.flags),
                SymbolValue::Template(_) => {
                    write!(f, "Template `{}` ({:?})", self.name, self.flags)
                }
                SymbolValue::Primitive(_) => {
                    write!(f, "Primititve `{}` {:?}", self.name, self.flags)
                }
                SymbolValue::Generic(_, _) => write!(f, "Generic `{}` {:?}", self.name, self.flags),
            }
        }
    }
}

impl TreeDisplay for Symbol {
    fn num_children(&self) -> usize {
        match &self.value {
            SymbolValue::Generic(_, ty) => match ty {
                GenericType::Generic(Some(t)) => t.len(),
                GenericType::Specialization(_) => 1,
                _ => 0,
            },
            _ => self.children.len(),
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match &self.value {
            SymbolValue::Generic(_, ty) => match ty {
                GenericType::Generic(Some(t)) => Some(&t[index]),
                GenericType::Specialization(t) => Some(t),
                _ => panic!(),
            },
            _ => Some(self.children.values().nth(index).unwrap()),
        }
    }

    // fn child_at_bx<'a>(&'a self, index: usize) -> Box<dyn TreeDisplay + 'a> {
    //     let p = self.children.values().nth(index).unwrap();
    //     Box::new(CreateParent(
    //         self.children.keys().nth(index).unwrap().clone(),
    //         vec![p],
    //     ))
    // }
}
