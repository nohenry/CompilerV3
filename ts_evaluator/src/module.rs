use std::{cell::RefCell, rc::Rc};

use colored::{ColoredString, Colorize};
use ts_errors::CodeGenError;
use ts_lexer::new_ast::Statement;
use ts_symbol::{Path, Symbol, SymbolFlags, SymbolValue, Type, Value};

#[derive(Debug, Clone, Copy)]
pub enum CodeGenPass {
    TemplateSymbols,
    TemplateSpecialization,
    TemplateValues,
    Symbols,
    SymbolsSpecialization,
    Values,
}

#[derive(Debug)]
pub struct Module {
    pub(super) name: String,
    pub(super) ast: Box<Statement>,
    pub(super) current_function: Rc<RefCell<Value>>,
    pub(super) jump_point: Rc<RefCell<Value>>,
    pub(super) errors: Rc<RefCell<Vec<CodeGenError>>>,
    pub(super) symbol_root: Rc<RefCell<Symbol>>,
    pub(super) current_symbol: Rc<RefCell<Path>>,
    pub(super) code_gen_pass: Rc<RefCell<CodeGenPass>>,
    pub(super) flags_to_apply: Rc<RefCell<SymbolFlags>>,
    pub(super) imports: Rc<RefCell<Vec<Path>>>,
}

impl Module {
    pub fn new(name: &String, ast: Box<Statement>, symbol_root: Rc<RefCell<Symbol>>) -> Module {
        let refr = symbol_root.clone();

        symbol_root
            .borrow_mut()
            .add_child(name, SymbolValue::Module);

        let p = Module {
            name: name.clone(),
            ast,
            current_function: Rc::new(RefCell::new(Value::Empty)),
            jump_point: Rc::new(RefCell::new(Value::Empty)),
            errors: Rc::new(RefCell::new(vec![])),
            symbol_root: refr,
            current_symbol: Rc::new(RefCell::new(vec![name.clone()])),
            code_gen_pass: Rc::new(RefCell::new(CodeGenPass::Symbols)),
            flags_to_apply: Rc::new(RefCell::new(SymbolFlags::empty())),
            imports: Rc::new(RefCell::new(vec![vec!["core".to_string()]])),
        };

        p
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn gen_core(&self) {
        self.add_symbol(
            &format!("number"),
            SymbolValue::Primitive(Type::Number(0.0)),
        );
        self.add_symbol(
            &format!("boolean"),
            SymbolValue::Primitive(Type::Boolean(false)),
        );

        self.add_symbol(
            &format!("string"),
            SymbolValue::Primitive(Type::String(String::from(""))),
        );

        self.flags_to_apply
            .borrow_mut()
            .set(SymbolFlags::EXPORT, true);
    }

    pub fn gen(&self) {
        self.gen_pass(CodeGenPass::TemplateSymbols);
        self.gen_pass(CodeGenPass::TemplateSpecialization);
        self.gen_pass(CodeGenPass::TemplateValues);

        self.gen_pass(CodeGenPass::Symbols);
        self.gen_pass(CodeGenPass::SymbolsSpecialization);
        self.gen_pass(CodeGenPass::Values);
    }

    pub fn print_errors(&self) {
        if self.errors.borrow().len() > 0 {
            println!(
                "{} `{}`: ",
                ColoredString::from("Module").bright_yellow(),
                self.name
            );
            for err in self.errors.borrow().iter() {
                println!(
                    "    {}: {}",
                    ColoredString::from("Error").bright_red(),
                    err.message
                );
            }
        }
    }

    pub fn gen_pass(&self, pass: CodeGenPass) {
        self.code_gen_pass.replace(pass);
        // self.gen_parse_node(&self.ast);
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
            c.add_child_flags(name, sym, *self.flags_to_apply.borrow());
        }

        self.current_symbol.borrow_mut().push(name.clone())
    }

    pub fn add_and_set_symbol_from_path(&self, path: &[String], name: &String, sym: SymbolValue) {
        let mut cur_sym = self.symbol_root.borrow_mut();
        let current = self.get_symbol_mut(&mut cur_sym, &path);

        if let Some(insert) = current {
            insert.add_child_flags(name, sym, *self.flags_to_apply.borrow());
        }

        let mut sad = Vec::from(path);
        sad.push(name.clone());
        self.current_symbol.replace(sad);
    }

    pub fn add_symbol(&self, name: &String, sym: SymbolValue) {
        let mut cur_sym = self.symbol_root.borrow_mut();
        let current = self.get_symbol_mut(&mut cur_sym, &self.current_symbol.borrow());

        if let Some(c) = current {
            c.add_child_flags(name, sym, *self.flags_to_apply.borrow());
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
        } else {
            for f in &*self.imports.borrow() {
                if let Some(s) = self.get_symbol(sym, f) {
                    if let Some(sym) = s.children.get(name) {
                        return Some(sym);
                    }
                }
            }
        }
        None
    }

    pub fn find_path_up_chain(&self, sym: &Symbol, chain: &[String], name: &String) -> Vec<String> {
        let current = self.get_symbol(sym, chain);
        if let Some(current) = current {
            if let Some(_) = current.children.get(name) {
                return Vec::from(chain);
            }
            return self.find_path_up_chain(sym, &chain[..chain.len() - 1], name);
        } else {
            for f in &*self.imports.borrow() {
                if let Some(s) = self.get_symbol(sym, f) {
                    if let Some(_) = s.children.get(name) {
                        let mut pt = f.to_vec();
                        pt.push(name.to_string());
                        return pt;
                    }
                }
            }
        }
        Vec::new()
    }

    pub fn find_up_chain_mut<'a>(
        &self,
        sym: &'a mut Symbol,
        chain: &[String],
        name: &String,
    ) -> Option<&'a mut Symbol> {
        let p = self.find_path_up_chain(sym, chain, name);
        let sym = self.get_symbol_mut(sym, &p);
        if let Some(sym) = sym {
            Some(sym)
        } else {
            None
        }
    }

    pub fn get_mangled_name(&self, name: &String) -> String {
        let mng = self.current_symbol.borrow().join("-");
        format!("{}-{}", mng, name)
    }

    pub fn get_mangled_name_with_path(&self, path: &[String], name: &String) -> String {
        let mng = path.join("-");
        format!("{}-{}", mng, name)
    }

    pub fn add_error(&self, message: String) {
        self.errors.borrow_mut().push(CodeGenError { message });
    }
}
