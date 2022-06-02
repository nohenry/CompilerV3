use std::{
    cell::{Ref, RefCell},
    fmt::Debug,
    rc::Rc,
};

use llvm_sys::{
    core::LLVMCreateBuilder,
    prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMModuleRef},
};

use dsl_lexer::ast::ParseNode;

use super::CodeGenError;
use dsl_symbol::{Symbol, SymbolValue};

pub struct Module {
    // pub(super) name: String,
    pub(super) ast: Box<ParseNode>,
    pub(super) module: LLVMModuleRef,
    pub(super) current_block: Rc<RefCell<LLVMBasicBlockRef>>,
    pub(super) builder: LLVMBuilderRef,
    pub(super) errors: Rc<RefCell<Vec<CodeGenError>>>,
    pub(super) symbol_root: Rc<RefCell<Symbol>>,
    pub(super) current_symbol: Vec<String>,
}

impl Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module").finish()
    }
}

impl Module {
    pub fn new(
        name: &String,
        ast: Box<ParseNode>,
        module: LLVMModuleRef,
        symbol_root: Rc<RefCell<Symbol>>,
    ) -> Module {
        let refr = symbol_root.clone();

        symbol_root
            .borrow_mut()
            .add_child(name, SymbolValue::Module);

        Module {
            ast,
            module,
            current_block: Rc::new(RefCell::new(std::ptr::null_mut())),
            builder: unsafe { LLVMCreateBuilder() },
            errors: Rc::new(RefCell::new(vec![])),
            symbol_root: refr,
            current_symbol: vec![name.clone()],
        }
    }

    pub fn get_module(&self) -> LLVMModuleRef {
        self.module
    }

    pub fn gen(&self) {
        let s = self.ast.clone();
        self.gen_parse_node(&s);
    }

    pub fn get_symbol<'a>(&self, sym: &'a Symbol, chain: &[String]) -> Option<&'a Symbol> {
        if chain.len() < 1 {
            return None;
        }
        let mut f = sym.children.get(&chain[0]).unwrap();
        for c in chain.iter().skip(1) {
            if let Some(sym) = f.children.get(c) {
                f = sym;
            } else {
                return None;
            }
        }
        Some(f)
    }

    pub fn get_symbol_mut<'a>(
        &self,
        sym: &'a mut Symbol,
        chain: &[String],
    ) -> Option<&'a mut Symbol> {
        if chain.len() < 1 {
            return None;
        }
        let mut f = sym.children.get_mut(&chain[0]).unwrap();
        for c in chain.iter().skip(1) {
            if let Some(sym) = f.children.get_mut(c) {
                f = sym;
            } else {
                return None;
            }
        }
        Some(f)
    }

    pub fn get_current(&self, f: impl Fn(Option<&Symbol>)) {
        let sym = self.symbol_root.borrow();
        let current = self.get_symbol(&sym, &self.current_symbol);
        (f)(current);
    }

    pub fn get_current_mut(&self, f: impl Fn(Option<&mut Symbol>)) {
        let mut sym = self.symbol_root.borrow_mut();
        let current = self.get_symbol_mut(&mut sym, &self.current_symbol);
        (f)(current);
    }

    pub fn find_symbol(&self, name: &String, f: impl Fn(Option<&Symbol>)) {
        let sym = self.symbol_root.borrow();
        let sym = self.find_up_chain(&sym, &self.current_symbol, name);
        (f)(sym);
    }

    pub fn find_up_chain<'a>(
        &self,
        sym: &'a Symbol,
        chain: &[String],
        name: &String,
    ) -> Option<&'a Symbol> {
        let current = self.get_symbol(sym, chain);
        if let Some(current) = current {
            if let Some(sym) = current.children.get(name) {
                return Some(sym);
            }
            if let Some(sym) = self.find_up_chain(sym, &chain[..chain.len() - 1], name) {
                return Some(sym);
            }
        }
        None
    }

    pub fn add_error(&self, message: String) {
        self.errors.borrow_mut().push(CodeGenError { message });
    }
}
