use std::{collections::HashMap, fmt::Display};

use dsl_util::{CreateParent, TreeDisplay};
use llvm_sys::{
    core::LLVMVoidType,
    prelude::{LLVMBasicBlockRef, LLVMTypeRef, LLVMValueRef},
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

impl Value {
    pub fn get_type(&self) -> &Type {
        match self {
            Self::Literal { literal_type, .. } => literal_type,
            Self::Variable { variable_type, .. } => variable_type,
            Self::Function { function_type, .. } => function_type,
            Self::Load { load_type, .. } => load_type,
            Self::Empty | Self::Instruction { .. } | Self::Block { .. } => {
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

#[derive(Debug, Clone, PartialEq)]
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
        parameters: HashMap<String, Type>,
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
            Self::Array { llvm_type, .. } => *llvm_type,
            Self::Unit { llvm_type, .. } => *llvm_type,
            Self::Reference { llvm_type, .. } => *llvm_type,
            Self::Function { llvm_type, .. } => *llvm_type,
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
            Type::Empty => write!(f, ""),
            Type::Unit { .. } => write!(f, "Unit"),
            Type::Integer { signed, .. } if !signed => write!(f, "Integer (unsigned)"),
            Type::Integer { .. } => write!(f, "Integer (signed)"),
            Type::Float { .. } => write!(f, "Float"),
            Type::Boolean { .. } => write!(f, "Boolean"),
            Type::Array { .. } => write!(f, "Array"),
            Type::Reference { .. } => write!(f, "Reference"),
            Type::Function { .. } => write!(f, "Function"),
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
