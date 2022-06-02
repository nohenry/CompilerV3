use std::collections::HashMap;

use llvm_sys::{
    core::LLVMVoidType,
    prelude::{LLVMTypeRef, LLVMValueRef},
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
    Instruction {
        llvm_value: LLVMValueRef,
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
            Self::Load { load_type, .. } => load_type,
            Self::Empty | Self::Instruction { .. } => panic!("Called on unkown value!"),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Value::Empty => true,
            _ => false,
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
            Self::Empty => panic!("Called on unkown value!"),
        }
    }

    pub fn unit_ty() -> Type {
        Type::Unit {
            llvm_type: unsafe { LLVMVoidType() },
        }
    }
}

#[derive(Debug)]
pub enum SymbolValue {
    Empty,
    Variable(Value),
    Funtion(Type),
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
