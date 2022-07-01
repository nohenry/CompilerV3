#![feature(linked_list_cursors)]
#![feature(extern_types)]
#![feature(crate_visibility_modifier)]
#![feature(let_else)]

use std::{
    cell::RefCell,
    collections::LinkedList,
    ffi::{CStr, CString},
    fs,
    os::raw::c_char,
    path::Path,
    rc::Rc,
};

use dsl_code_generation::module::{CodeGenPass, Module};
use dsl_symbol::Symbol;
use llvm_sys::{
    bit_writer::LLVMWriteBitcodeToFile,
    core::{LLVMModuleCreateWithName, LLVMPrintModuleToFile},
    prelude::LLVMModuleRef,
    target::{
        LLVM_InitializeAllAsmParsers, LLVM_InitializeAllAsmPrinters, LLVM_InitializeAllTargetMCs,
        LLVM_InitializeAllTargets,
    },
    target_machine::{
        LLVMCreateTargetMachine, LLVMGetDefaultTargetTriple, LLVMGetFirstTarget,
        LLVMGetTargetFromTriple, LLVMTargetMachineEmitToFile, LLVMTargetMachineRef, LLVMTargetRef,
    },
};

use dsl_util::{c_str, TreeDisplay};

mod builtins;

fn create_target_machine() -> LLVMTargetMachineRef {
    unsafe {
        let triple = LLVMGetDefaultTargetTriple();
        let mut target: LLVMTargetRef = std::ptr::null_mut();
        let mut error: *mut c_char = std::ptr::null_mut();
        LLVMGetTargetFromTriple(triple, &mut target, &mut error);
        let target = LLVMGetFirstTarget();

        LLVMCreateTargetMachine(
            target,
            triple,
            c_str!("x86-64") as _,
            c_str!("") as _,
            llvm_sys::target_machine::LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
            llvm_sys::target_machine::LLVMRelocMode::LLVMRelocDefault,
            llvm_sys::target_machine::LLVMCodeModel::LLVMCodeModelDefault,
        )
    }
}

fn llvm_bool<F>(f: F) -> Result<(), String>
where
    F: FnOnce(&mut *mut i8) -> i32,
{
    let mut error_str = std::ptr::null_mut();
    let res = f(&mut error_str);
    if res == 1 {
        let err = unsafe { CStr::from_ptr(error_str) };
        Err(err.to_string_lossy().into_owned())
        //LLVMDisposeMessage(error_str);
    } else {
        Ok(())
    }
}

enum EmitType {
    Asm,
    Object,
    IR,
    Bitcode,
}

fn emit(
    module: LLVMModuleRef,
    target_machine: LLVMTargetMachineRef,
    filename: &Path,
    emit_type: EmitType,
) {
    let outfile = match emit_type {
        EmitType::Asm => filename.with_extension("s"),
        EmitType::Object => filename.with_extension("o"),
        EmitType::IR => filename.with_extension("ll"),
        EmitType::Bitcode => filename.with_extension("bc"),
    };

    let output_file = CString::new(outfile.to_str().unwrap()).expect("invalid file");
    unsafe {
        match emit_type {
            EmitType::Asm => {
                llvm_bool(|error| {
                    LLVMTargetMachineEmitToFile(
                        target_machine,
                        module,
                        output_file.as_ptr() as *mut i8,
                        llvm_sys::target_machine::LLVMCodeGenFileType::LLVMAssemblyFile,
                        error,
                    )
                })
                .expect("Unable to emit assembly file");
            }
            EmitType::Object => {
                llvm_bool(|error| {
                    LLVMTargetMachineEmitToFile(
                        target_machine,
                        module,
                        output_file.as_ptr() as *mut i8,
                        llvm_sys::target_machine::LLVMCodeGenFileType::LLVMObjectFile,
                        error,
                    )
                })
                .expect("Unable to emit object file");
            }
            EmitType::Bitcode => {
                llvm_bool(|_error| LLVMWriteBitcodeToFile(module, output_file.as_ptr() as *mut i8))
                    .expect("Unable to emit bitcode");
            }
            EmitType::IR => {
                llvm_bool(|error| {
                    LLVMPrintModuleToFile(module, output_file.as_ptr() as *mut i8, error)
                })
                .expect("Unable to emit ir");
            }
        }

        // let b =
    }
}

fn compile_file(
    contents: &String,
    module_name: &String,
    module: LLVMModuleRef,
    symbols: Rc<RefCell<Symbol>>,
) -> Module {
    let tokens = dsl_lexer::lex(&contents).unwrap();
    let mut ltokens = LinkedList::new();
    for tok in &tokens {
        ltokens.push_back(tok);
    }

    let parser = dsl_parser::Parser::parse_from_tokens(&ltokens);
    parser.print_errors();

    let ast = parser.get_ast();
    println!("{}", ast.format());

    let module = dsl_code_generation::module::Module::new(
        &module_name.to_string(),
        Box::new(ast.clone()),
        module,
        symbols,
    );

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
    let target_machine = unsafe {
        LLVM_InitializeAllTargets();
        LLVM_InitializeAllTargetMCs();
        LLVM_InitializeAllAsmPrinters();
        LLVM_InitializeAllAsmParsers();

        create_target_machine()
    };

    let symbols = Symbol::root();

    let module = unsafe { LLVMModuleCreateWithName(CString::new("test")?.as_ptr() as *const i8) };

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
                    module,
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

    if let Some(core) = core_module {
        core.gen_main_fn();
    }

    let sym = symbols.borrow();
    println!("{}", sym.format());

    let filename = path.file_name().unwrap();
    emit(
        module,
        target_machine,
        Path::new(filename.to_str().unwrap()),
        EmitType::IR,
    );
    emit(
        module,
        target_machine,
        Path::new(filename.to_str().unwrap()),
        EmitType::Object,
    );

    Ok(())
}
