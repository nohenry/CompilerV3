use std::{cell::RefCell, fmt::Debug, rc::Rc};

use colored::{ColoredString, Colorize};
use dsl_llvm::IRBuilder;
use llvm_sys::{core::LLVMCreateBuilder, prelude::LLVMModuleRef};

use dsl_lexer::ast::ParseNode;

use dsl_errors::CodeGenError;
use dsl_symbol::{Symbol, SymbolValue, Value};

pub enum CodeGenPass {
    Symbols,
    SymbolsSpecialization,
    Values,
}

pub struct Module {
    pub(super) ast: Box<ParseNode>,
    pub(super) module: LLVMModuleRef,
    pub(super) current_block: Rc<RefCell<Value>>,
    pub(super) current_function: Rc<RefCell<Value>>,
    pub(super) jump_point: Rc<RefCell<Value>>,
    pub(super) builder: IRBuilder,
    pub(super) errors: Rc<RefCell<Vec<CodeGenError>>>,
    pub(super) symbol_root: Rc<RefCell<Symbol>>,
    pub(super) current_symbol: Rc<RefCell<Vec<String>>>,
    pub(super) code_gen_pass: Rc<RefCell<CodeGenPass>>,
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

        let p = Module {
            ast,
            module,
            current_block: Rc::new(RefCell::new(Value::Empty)),
            current_function: Rc::new(RefCell::new(Value::Empty)),
            jump_point: Rc::new(RefCell::new(Value::Empty)),
            builder: IRBuilder::new(unsafe { LLVMCreateBuilder() }, module),
            errors: Rc::new(RefCell::new(vec![])),
            symbol_root: refr,
            current_symbol: Rc::new(RefCell::new(vec![name.clone()])),
            code_gen_pass: Rc::new(RefCell::new(CodeGenPass::Symbols)),
        };
        p.add_symbol(
            &"print".to_string(),
            SymbolValue::Funtion(
                p.builder
                    .add_function(
                        p.builder.get_fn(
                            p.builder.get_unit(),
                            &vec![(
                                "str".to_string(),
                                p.builder.get_ptr(&p.builder.get_uint_8()),
                            )],
                        ),
                        "printf".to_string(),
                        module,
                    )
                    .unwrap(),
            ),
        );
        p
    }

    pub fn get_module(&self) -> LLVMModuleRef {
        self.module
    }

    pub fn gen(&self) {
        self.gen_parse_node(&self.ast);
        self.code_gen_pass
            .replace(CodeGenPass::SymbolsSpecialization);
        self.gen_parse_node(&self.ast);
        self.code_gen_pass.replace(CodeGenPass::Values);
        self.gen_parse_node(&self.ast);

        for err in self.errors.borrow().iter() {
            println!(
                "{}: {}",
                ColoredString::from("Error").bright_red(),
                err.message
            );
        }
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

    pub fn add_and_set_symbol(&self, name: &String, sym: SymbolValue) {
        let mut cur_sym = self.symbol_root.borrow_mut();
        let current = self.get_symbol_mut(&mut cur_sym, &self.current_symbol.borrow());

        if let Some(c) = current {
            c.add_child(name, sym);
        }

        self.current_symbol.borrow_mut().push(name.clone())
    }

    pub fn add_and_set_symbol_from_path(&self, path: &[String], name: &String, sym: SymbolValue) {
        let mut cur_sym = self.symbol_root.borrow_mut();
        let current = self.get_symbol_mut(&mut cur_sym, &path);

        if let Some(insert) = current {
            insert.add_child(name, sym);
        }

        let mut sad = Vec::from(path);
        sad.push(name.clone());
        self.current_symbol.replace(sad);
    }

    pub fn add_symbol(&self, name: &String, sym: SymbolValue) {
        let mut cur_sym = self.symbol_root.borrow_mut();
        let current = self.get_symbol_mut(&mut cur_sym, &self.current_symbol.borrow());

        if let Some(c) = current {
            c.add_child(name, sym);
        }
    }

    pub fn get_next_name(&self, path: &[String], name: String) -> String {
        let cur_sym = self.symbol_root.borrow();
        let current = self.get_symbol(&cur_sym, &path);

        if let Some(insert) = current {
            let mut i = 1;
            if insert.children.contains_key(&name) {
                let mut next_name = format!("{}{}", name, i);
                while insert.children.contains_key(&next_name) {
                    i += 1;
                    next_name = format!("{}{}", name, i);
                }
                next_name
            } else {
                name
            }
        } else {
            name
        }
    }

    pub fn pop_symbol(&self) {
        let new = match &*self.current_symbol.borrow().as_slice() {
            [vals @ .., _] => Vec::from(vals),
            _ => Vec::new(),
        };
        self.current_symbol.replace(new);
    }

    pub fn get_current(&self, f: impl Fn(Option<&Symbol>)) {
        let sym = self.symbol_root.borrow();
        let current = self.get_symbol(&sym, &self.current_symbol.borrow());
        (f)(current);
    }

    pub fn get_current_mut(&self, f: impl Fn(Option<&mut Symbol>)) {
        let mut sym = self.symbol_root.borrow_mut();
        let current = self.get_symbol_mut(&mut sym, &self.current_symbol.borrow());
        (f)(current);
    }

    pub fn find_symbol(&self, name: &String, f: impl Fn(Option<&Symbol>)) {
        let sym = self.symbol_root.borrow();
        let sym = self.find_up_chain(&sym, &self.current_symbol.borrow(), name);
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

    pub fn get_mangled_name(&self, name: &String) -> String {
        let mng = self.current_symbol.borrow().join("-");
        format!("{}-{}", mng, name)
    }

    pub fn add_error(&self, message: String) {
        self.errors.borrow_mut().push(CodeGenError { message });
    }
}
