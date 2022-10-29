#![feature(linked_list_cursors)]
#![feature(extern_types)]
#![feature(crate_visibility_modifier)]
#![feature(let_else)]

use std::{cell::RefCell, collections::LinkedList, fs, path::Path, rc::Rc};

use ts_evaluator::module::{CodeGenPass, Module};
use ts_lexer::new_lexer::Lexer;
use ts_symbol::Symbol;

use ts_util::TreeDisplay;

mod builtins;

fn compile_file(contents: &String, module_name: &String, symbols: Rc<RefCell<Symbol>>) -> Module {
    let lexer = Lexer::new(&contents);
    let tokens = lexer.lex();

    for token in tokens {
        println!("Token: {}", &token);
    }


    // let mut ltokens = LinkedList::new();
    // for tok in &tokens {
    //     ltokens.push_back(tok);
    // }

    // let parser = ts_parser::new_parser::Parser::parse_from_tokens(&ltokens);
    // parser.print_errors();

    // let ast = parser.get_file().statements;
    // for st in ast {
    //     println!("{:?}", st);
    // }

    let module = Module::new(&module_name.to_string(), Box::new(ts_lexer::new_ast::Statement::Debugger), symbols);

    module
}

fn gen_files_pass(files: &Vec<Option<Module>>, pass: CodeGenPass) {
    for file in files {
        if let Some(file) = file {
            file.gen_pass(pass);
        }
    }
}

fn main() -> std::io::Result<()> {
    let symbols = Symbol::root();

    let symbols = Rc::new(RefCell::new(symbols));

    let path = Path::new("test");

    let files: Vec<Option<Module>> = path
        .read_dir()?
        .map(|file| {
            let Ok(file) = file else {
            return None;
        };
            let Ok(ty) = file.file_type() else {
            return None;
        };
            if ty.is_file() {
                if let Some(p) = file.path().extension() {
                    if p != "ts" && p != "tsx" {
                        return None;
                    }
                } else {
                    return None;
                }

                let path = file.path();

                let contents = fs::read_to_string(&path).expect("Unable to read file!");
                let module_name = path.file_stem().unwrap().to_str().unwrap();

                return Some(compile_file(
                    &contents,
                    &module_name.to_string(),
                    symbols.clone(),
                ));
            }
            None
        })
        .collect();

    Ok(())

    // let contents = fs::read_to_string(&path).expect("Unable to read file!");
}
/*
fn main() -> std::io::Result<()> {
    let symbols = Symbol::root();

    let symbols = Rc::new(RefCell::new(symbols));

    let path = Path::new("test");

    let files: Vec<Option<Module>> = path
        .read_dir()?
        .map(|file| {
            let Ok(file) = file else {
            return None;
        };
            let Ok(ty) = file.file_type() else {
            return None;
        };
            if ty.is_file() {
                if let Some(p) = file.path().extension() {
                    if p != "dsl" {
                        return None;
                    }
                } else {
                    return None;
                }
                let path = file.path();

                let contents = fs::read_to_string(&path).expect("Unable to read file!");
                let module_name = path.file_stem().unwrap().to_str().unwrap();

                return Some(compile_file(
                    &contents,
                    &module_name.to_string(),
                    symbols.clone(),
                ));
            }
            None
        })
        .collect();

    let core_module = files
        .iter()
        .filter_map(|f| f.as_ref())
        .find(|f| f.get_name() == "core");

    if let Some(core) = core_module {
        core.gen_core();
    }

    if let Some(s) = symbols.borrow_mut().children.get_mut("core") {
        builtins::add_builtins(s);
    }

    gen_files_pass(&files, CodeGenPass::TemplateSymbols);
    gen_files_pass(&files, CodeGenPass::TemplateSpecialization);
    gen_files_pass(&files, CodeGenPass::TemplateValues);

    gen_files_pass(&files, CodeGenPass::Symbols);
    gen_files_pass(&files, CodeGenPass::SymbolsSpecialization);
    gen_files_pass(&files, CodeGenPass::Values);

    for file in &files {
        if let Some(file) = file {
            file.print_errors()
        }
    }

    let sym = symbols.borrow();
    println!("{}", sym.format());

    Ok(())
}

*/
