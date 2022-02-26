use std::{collections::HashMap, cell::RefCell, rc::Rc};

use crate::ast::ParseNode;

#[derive(Debug, Clone)]
pub enum SymbolType {
    Global,
    Function(Box<ParseNode>),
    Template,
    Variable
}

#[derive(Debug, Clone)]
pub struct Symbol {
    name: String,
    children: HashMap<String, Rc<RefCell<Symbol>>>,
    pub symbol_type: SymbolType,
}

impl Symbol {
    pub fn new(name: String) -> Self {
        Symbol {
            name,
            children: HashMap::default(),
            symbol_type: SymbolType::Global,
        }
    }

    pub fn new_from_symbol_type(name: String, symbol: SymbolType) -> Symbol {
        Symbol {
            name,
            children: HashMap::default(),
            symbol_type: symbol,
        }
    }

    pub fn insert(&mut self, name: &String, symbol: SymbolType) {
        self.children.insert(
            name.clone(),
            Rc::new(RefCell::new(Symbol::new_from_symbol_type(name.clone(), symbol))),
        );
    }

    pub fn find(&self, name: &String) -> Option<&Rc<RefCell<Symbol>>> {
        self.children.get(name)
    }

    // pub fn find_mut(&mut self, name: &String) -> Option<&mut Symbol> {
    //     self.children.get_mut(name)
    // }
}
