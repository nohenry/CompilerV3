use std::{cell::RefCell, collections::HashMap, rc::Rc};

use llvm_sys::prelude::{LLVMTypeRef, LLVMValueRef};

use self::module::Module;

mod expression;
mod literal;
pub mod module;
mod parse_node;
mod value_type;

struct CodeGenError {
    message: String,
}

#[derive(Debug, Clone)]
pub struct Value {
    llvm_value: LLVMValueRef,
    value_type: Box<Type>,
}

impl Value {
    fn new(llvm_value: LLVMValueRef, value_type: Box<Type>) -> Value {
        Value {
            llvm_value,
            value_type,
        }
    }

    fn empty() -> Value {
        Value {
            llvm_value: std::ptr::null_mut(),
            value_type: Box::new(Type::empty()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Type {
    llvm_type: LLVMTypeRef,
    signed: bool,
}

impl Type {
    fn new(llvm_type: LLVMTypeRef, signed: bool) -> Type {
        Type { llvm_type, signed }
    }

    fn empty() -> Type {
        Type {
            llvm_type: std::ptr::null_mut(),
            signed: false,
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
    // pub parent: Option<Rc<RefCell<Symbol>>>,
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

    // pub fn find_symbol_up(&self, name: &String) -> Option<Rc<RefCell<Symbol>>> {
    //     if let Some(sym) = self.children.get(name) {
    //         return Some(sym.clone());
    //     }
    //     if let Some(sym) = &self.parent {
    //         if let Some(sym) = Symbol::find_symbol_up(&sym.borrow_mut(), name) {
    //             return Some(sym);
    //         }
    //     }
    //     None
    // }
}
