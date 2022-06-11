#![feature(linked_list_cursors)]
#![feature(extern_types)]
#![feature(crate_visibility_modifier)]
use std::{
    cell::RefCell,
    collections::LinkedList,
    ffi::{CStr, CString},
    fs,
    os::raw::c_char,
    path::Path,
    rc::Rc,
};

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
                llvm_bool(|error| LLVMWriteBitcodeToFile(module, output_file.as_ptr() as *mut i8))
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

fn main() {
    let target_machine = unsafe {
        LLVM_InitializeAllTargets();
        LLVM_InitializeAllTargetMCs();
        LLVM_InitializeAllAsmPrinters();
        LLVM_InitializeAllAsmParsers();

        create_target_machine()
    };

    let symbols = Symbol::root();

    let path = Path::new("test/test.dsl");

    let contents = fs::read_to_string(path).expect("Unable to read file!");

    let tokens = dsl_lexer::lex(&contents).unwrap();
    let mut ltokens = LinkedList::new();
    for tok in &tokens {
        ltokens.push_back(tok);
    }

    let ast = dsl_parser::parse_from_tokens(&ltokens).unwrap();
    println!("{}", ast.format());

    let name = CString::new(path.to_str().unwrap()).expect("Unable to create model name");

    let symbols = Rc::new(RefCell::new(symbols));

    unsafe {
        let module = LLVMModuleCreateWithName(name.as_ptr());
        let name = name.into_string().unwrap();

        let module =
            dsl_code_generation::module::Module::new(&name, Box::new(ast), module, symbols.clone());

        module.gen();

        let sym = symbols.borrow();
        println!("{}", sym.format());

        let filename = path.file_name().unwrap();
        emit(
            module.get_module(),
            target_machine,
            Path::new(filename.to_str().unwrap()),
            EmitType::IR,
        );
    }
}
