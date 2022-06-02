use std::collections::HashMap;

use llvm_sys::{
    core::LLVMVoidType,
    prelude::{LLVMTypeRef, LLVMValueRef},
};

mod expression;
mod literal;
pub mod module;
mod parse_node;
mod value_type;

struct CodeGenError {
    message: String,
}

