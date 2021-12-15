use std::fs;

use llvm::{LLVMFunctionType, LLVMInt32Type, LLVMWriteBitcodeToFile, c_str};

mod ast;
mod lexer;
mod parser;
mod trie;



fn main() {
    unsafe {
        let module = llvm::LLVMModuleCreateWithName(c_str!("Potato"));
        let param_types = [LLVMInt32Type(), LLVMInt32Type()];
        let return_type = LLVMFunctionType(LLVMInt32Type(), param_types.as_ptr(), 2, false);
        let sum = llvm::LLVMAddFunction(module, c_str!("sum"), return_type);

        LLVMWriteBitcodeToFile(module, c_str!("out.bc"));
    }

    let contents = fs::read_to_string("test/test.dsl").expect("Unable to read file!");
    let tokens = lexer::lex(&contents).unwrap();
    println!("{:#?}", tokens);
    let ast = parser::parse_from_tokens(&tokens).unwrap();
    println!("{:#?}", ast);
    let o = !if true { false } else { true };
}
