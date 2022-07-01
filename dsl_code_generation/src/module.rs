use std::{cell::RefCell,  fmt::Debug, rc::Rc};

use colored::{ColoredString, Colorize};
use dsl_llvm::IRBuilder;
use llvm_sys::{core::LLVMCreateBuilder, prelude::LLVMModuleRef};

use dsl_lexer::ast::ParseNode;

use dsl_errors::CodeGenError;
use dsl_symbol::{Symbol, SymbolFlags, SymbolValue, Value};

#[derive(Clone, Copy)]
pub enum CodeGenPass {
    TemplateSymbols,
    TemplateSpecialization,
    TemplateValues,
    Symbols,
    SymbolsSpecialization,
    Values,
}

pub struct Module {
    pub(super) name: String,
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
    pub(super) flags_to_apply: Rc<RefCell<SymbolFlags>>,
    pub(super) imports: Rc<RefCell<Vec<Vec<String>>>>,
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
            name: name.clone(),
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
            flags_to_apply: Rc::new(RefCell::new(SymbolFlags::empty())),
            imports: Rc::new(RefCell::new(vec![vec!["core".to_string()]])),
        };

        p
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn gen_main_fn(&self) {
        let main_fn = self
            .builder
            .add_function(
                IRBuilder::get_fn(IRBuilder::get_unit(), &vec![]),
                "main".to_string(),
                self.module,
            )
            .unwrap();
        let block = self.builder.append_block(&main_fn).unwrap();

        self.current_block.replace(block);
        self.builder.set_position_end(&self.current_block.borrow());

        {
            let root = self.symbol_root.borrow();
            let sym = self.get_symbol(&root, &["test".into(), "main".into()]);
            if let Some(Symbol {
                value: SymbolValue::Function(val),
                ..
            }) = sym
            {
                match self.builder.create_fn_call(val, Vec::new()) {
                    Ok(_) => (),
                    Err(e) => {
                        self.errors.borrow_mut().push(e);
                    }
                }
            } else {
                self.add_error("Unable to find main function!".into())
            }
        }
        self.builder.create_ret_void();

        self.current_function.replace(main_fn.clone());
        self.add_symbol(&"main".to_string(), SymbolValue::Function(main_fn));
    }

    pub fn gen_core(&self) {
        for i in [8, 16, 32, 64] {
            self.add_symbol(
                &format!("int{}", i),
                SymbolValue::Primitive(IRBuilder::get_int(i)),
            );
            self.add_symbol(
                &format!("uint{}", i),
                SymbolValue::Primitive(IRBuilder::get_uint(i)),
            );
        }
        self.add_symbol(
            &format!("bool"),
            SymbolValue::Primitive(IRBuilder::get_bool()),
        );

        self.add_symbol(
            &format!("char"),
            SymbolValue::Primitive(IRBuilder::get_uint_8()),
        );

        self.add_symbol(
            &format!("float32"),
            SymbolValue::Primitive(IRBuilder::get_float32()),
        );
        self.add_symbol(
            &format!("float64"),
            SymbolValue::Primitive(IRBuilder::get_float64()),
        );

        self.flags_to_apply
            .borrow_mut()
            .set(SymbolFlags::EXPORT, true);

        self.add_symbol(
            &"print".to_string(),
            SymbolValue::Function(
                self.builder
                    .add_function(
                        IRBuilder::get_fn(
                            IRBuilder::get_unit(),
                            &vec![(
                                "str".to_string(),
                                IRBuilder::get_ptr(&IRBuilder::get_uint_8(), false),
                            )],
                        ),
                        "printf".to_string(),
                        self.module,
                    )
                    .unwrap(),
            ),
        );
    }

    pub fn get_module(&self) -> LLVMModuleRef {
        self.module
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
        self.gen_parse_node(&self.ast);
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
