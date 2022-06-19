use std::{collections::HashMap, fmt::Display};

use dsl_lexer::ast::{FunctionSignature, ParseNode};
use dsl_util::{CreateParent, TreeDisplay, NULL_STR};
use linked_hash_map::LinkedHashMap;
use llvm_sys::{
    core::{
        LLVMBuildIntCast2, LLVMBuildLoad2, LLVMGetIntTypeWidth, LLVMGetTypeKind, LLVMPointerType,
        LLVMVoidType,
    },
    prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMTypeRef, LLVMValueRef},
};

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
    },
    Function {
        llvm_value: LLVMValueRef,
        function_type: Type,
    },
    FunctionTemplate {
        path: Vec<String>,
        ty: FunctionSignature,
        body: Box<ParseNode>,
        ty_params: LinkedHashMap<String, Option<Vec<Type>>>,
        types: HashMap<Vec<String>, Vec<String>>,
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
            Self::Load { load_type, .. } => load_type,
            Self::Empty
            | Self::Instruction { .. }
            | Self::Block { .. }
            | Self::FunctionTemplate { .. } => {
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

    pub fn has_value(&self) -> bool {
        match self {
            Value::Empty | Value::Instruction { .. } | Value::Block { .. } => false,
            _ => true,
        }
    }

    pub fn get_value(&self, builder: LLVMBuilderRef) -> Result<LLVMValueRef, ()> {
        match self {
            Value::Empty
            | Value::Instruction { .. }
            | Value::Block { .. }
            | Value::FunctionTemplate { .. } => Err(()),
            Value::Function { llvm_value, .. } => Ok(*llvm_value),
            Value::Variable {
                llvm_value,
                variable_type,
            } => unsafe {
                Ok(LLVMBuildLoad2(
                    builder,
                    variable_type.get_type(),
                    *llvm_value,
                    NULL_STR,
                ))
            },
            Value::Literal { llvm_value, .. } => Ok(*llvm_value),
            Value::Load { llvm_value, .. } => Ok(*llvm_value),
        }
    }

    pub fn weak_cast(&self, to_type: &Type, builder: LLVMBuilderRef) -> Result<Value, bool> {
        if self.get_type() == to_type {
            return Err(false);
        }
        match (self, to_type) {
            (
                Value::Literal {
                    llvm_value,
                    literal_type:
                        Type::Integer {
                            llvm_type: ltype, ..
                        },
                },
                Type::Integer {
                    llvm_type: rtype,
                    signed,
                },
            ) => {
                if *ltype != *rtype {
                    return Ok(Value::Literal {
                        llvm_value: unsafe {
                            LLVMBuildIntCast2(
                                builder,
                                *llvm_value,
                                *rtype,
                                if *signed { 1 } else { 0 },
                                NULL_STR,
                            )
                        },
                        literal_type: to_type.clone(),
                    });
                }
            }
            _ => (),
        }
        Err(true)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Literal { literal_type, .. } => write!(f, "{}", literal_type),
            Value::Variable { variable_type, .. } => write!(f, "{}", variable_type),
            Value::Function { function_type, .. } => write!(f, "{}", function_type),
            Value::Load { load_type, .. } => write!(f, "{}", load_type),
            Value::Instruction { .. } => write!(f, "Instruction"),
            Value::Block { .. } => write!(f, "Block"),
            Value::FunctionTemplate { .. } => write!(f, "Function Template"),
            Value::Empty => write!(f, "Empty"),
        }
    }
}

impl TreeDisplay for Value {
    fn num_children(&self) -> usize {
        match self {
            Value::Literal { literal_type, .. } => literal_type.num_children(),
            Value::Variable { variable_type, .. } => variable_type.num_children(),
            Value::Function { function_type, .. } => function_type.num_children(),
            Value::Load { load_type, .. } => load_type.num_children(),
            _ => 0,
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match self {
            Value::Literal { literal_type, .. } => literal_type.child_at(index),
            Value::Variable { variable_type, .. } => variable_type.child_at(index),
            Value::Function { function_type, .. } => function_type.child_at(index),
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
    },
    Function {
        llvm_type: LLVMTypeRef,
        parameters: LinkedHashMap<String, Type>,
        return_type: Box<Type>,
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
            Self::Empty => panic!("Called on unkown value!"),
        }
    }

    pub fn unit_ty() -> Type {
        Type::Unit {
            llvm_type: unsafe { LLVMVoidType() },
        }
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
            _ => 0,
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match self {
            Type::Array { base_type, .. } => Some(base_type.as_ref()),
            Type::Reference { base_type, .. } => Some(base_type.as_ref()),
            Type::Function { return_type, .. } if index == 0 => Some(return_type.as_ref()),
            Type::Function { .. } => None,
            _ => panic!(),
        }
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<dyn TreeDisplay + 'a> {
        match self {
            Type::Function { parameters, .. } => Box::new(CreateParent(
                parameters.keys().nth(index - 1).unwrap().clone(),
                vec![parameters.values().nth(index - 1).unwrap()],
            )),
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
                },
                Self::Reference {
                    llvm_type: r_llvm_type,
                    base_type: r_base_type,
                },
            ) => l_llvm_type == r_llvm_type && l_base_type == r_base_type,
            (
                Self::Function {
                    llvm_type: l_llvm_type,
                    parameters: l_parameters,
                    return_type: l_return_type,
                },
                Self::Function {
                    llvm_type: r_llvm_type,
                    parameters: r_parameters,
                    return_type: r_return_type,
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

#[derive(Debug)]
pub enum SymbolValue {
    Empty,
    Variable(Value),
    Funtion(Value),
    Field(Type),
    Template(Type),
    Action(Type),
    Spec(Type),
    Alias(Type),
    Generic(Type, Option<Vec<Type>>),
    Module,
}

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub value: SymbolValue,
    pub children: HashMap<String, Symbol>,
}

impl Symbol {
    pub fn root() -> Symbol {
        Symbol {
            name: String::from("root"),
            value: SymbolValue::Empty,
            children: HashMap::new(),
        }
    }

    pub fn new(name: String, value: SymbolValue) -> Symbol {
        Symbol {
            name,
            value,
            children: HashMap::new(),
        }
    }

    pub fn add_child(&mut self, name: &String, value: SymbolValue) {
        self.children
            .insert(name.clone(), Symbol::new(name.clone(), value));
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            SymbolValue::Variable(_) => write!(f, "Variable `{}`", self.name),
            SymbolValue::Action(_) => write!(f, "Action `{}`", self.name),
            SymbolValue::Alias(_) => write!(f, "Alias `{}`", self.name),
            SymbolValue::Empty => write!(f, "{}", self.name),
            SymbolValue::Field(_) => write!(f, "Field `{}`", self.name),
            SymbolValue::Funtion(_) => write!(f, "Function `{}`", self.name),
            SymbolValue::Module => write!(f, "Module `{}`", self.name),
            SymbolValue::Spec(_) => write!(f, "Spec `{}`", self.name),
            SymbolValue::Template(_) => write!(f, "Template `{}`", self.name),
            SymbolValue::Generic(_, _) => write!(f, "Generic `{}`", self.name),
        }
    }
}

impl TreeDisplay for Symbol {
    fn num_children(&self) -> usize {
        self.children.len()
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        Some(self.children.values().nth(index).unwrap())
    }

    // fn child_at_bx<'a>(&'a self, index: usize) -> Box<dyn TreeDisplay + 'a> {
    //     let p = self.children.values().nth(index).unwrap();
    //     Box::new(CreateParent(
    //         self.children.keys().nth(index).unwrap().clone(),
    //         vec![p],
    //     ))
    // }
}
