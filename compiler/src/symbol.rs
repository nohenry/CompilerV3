use std::{borrow::Borrow, cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use crate::{
    ast::{FunctionType, ParseNode, Type},
    value::Value,
};

#[derive(Clone)]
pub enum SymbolType {
    Global,
    Function(Option<Box<ParseNode>>, FunctionType),
    NativeFunction(Box<fn(Vec<Value>)>, Vec<Type>, Type),
    Template,
    Module,
    Variable(Value, Type),
}

impl Debug for SymbolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Global => write!(f, "Global"),
            Self::Function(arg0, arg1) => {
                f.debug_tuple("Function").field(arg0).field(arg1).finish()
            }
            Self::NativeFunction(_, _, _) => f.debug_tuple("NativeFunction").finish(),
            Self::Template => write!(f, "Template"),
            Self::Variable(arg0, arg1) => {
                f.debug_tuple("Variable").field(arg0).field(arg1).finish()
            }
            Self::Module => write!(f, "Module"),
        }
    }
}

pub type SymRef = Rc<RefCell<Symbol>>;

#[derive(Clone)]
pub struct Symbol {
    name: String,
    children: HashMap<String, SymRef>,
    pub symbol_type: SymbolType,
    parent: Option<SymRef>,
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Symbol")
            .field("name", &self.name)
            .field("children", &self.children)
            .field("symbol_type", &self.symbol_type)
            .finish()
    }
}

impl Symbol {
    pub fn new(name: String) -> Self {
        Symbol {
            name,
            children: HashMap::default(),
            symbol_type: SymbolType::Global,
            parent: None,
        }
    }

    pub fn new_from_symbol_type(parent: SymRef, name: String, symbol: SymbolType) -> Symbol {
        Symbol {
            name,
            children: HashMap::default(),
            symbol_type: symbol,
            parent: Some(parent),
        }
    }

    pub fn insert(slf: SymRef, name: &String, symbol: SymbolType) -> SymRef {
        let rc = Rc::new(RefCell::new(Symbol::new_from_symbol_type(
            slf.clone(),
            name.clone(),
            symbol,
        )));
        slf.borrow_mut()
            .children
            .insert(name.clone(), Rc::clone(&rc));
        rc
    }

    pub fn find(&self, name: &String) -> Option<&SymRef> {
        self.children.get(name)
    }

    pub fn find_in_scope(slf: SymRef, name: &String) -> Option<SymRef> {
        match slf.as_ref().borrow().find(name) {
            Some(s) => Some(s.clone()),
            None => match slf.as_ref().borrow().parent {
                Some(ref p) => Symbol::find_in_scope(p.clone(), name),
                None => None,
            },
        }
    }

    pub fn parent(&self) -> Option<SymRef> {
        self.parent.clone()
    }
}
