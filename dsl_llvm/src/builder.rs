// pub struct BasicBlock {
//     basic_block: LLVMBasicBlockRef,
// }

// impl BasicBlock {
//     pub fn create(name: &str, context: &Context, function: &Function) -> Self {
//         let basic_block = unsafe {
//             LLVMAppendBasicBlockInContext(
//                 context.context(),
//                 function.function(),
//                 &name.as_bytes()[0] as *const u8,
//             )
//         };
//         BasicBlock { basic_block }
//     }

//     pub fn basic_block(&self) -> LLVMBasicBlockRef {
//         self.basic_block
//     }
// }

// pub struct Builder {
//     builder: LLVMBasicBlockRef,
// }

// impl Builder {
//     pub fn create(context: &Context) -> Self {
//         let builder = unsafe { LLVMCreateBuilderInContext(context.context()) };
//         Builder { builder }
//     }

//     pub fn position_at_end(&self, basic_block: &BasicBlock) {
//         unsafe { LLVMPositionBuilderAtEnd(self.builder, basic_block.basic_block()) }
//     }
// }

use dsl_util::NULL_STR;
use llvm_sys::{
    core::{LLVMPositionBuilder, LLVMPositionBuilderAtEnd, LLVMBuildAlloca},
    prelude::{LLVMBasicBlockRef, LLVMValueRef, LLVMBuilderRef},
};

use dsl_symbol::{Type, Value};

pub struct IRBuilder {
    builder: LLVMBuilderRef,
}

impl IRBuilder {
    pub fn new(builder: LLVMBuilderRef) -> IRBuilder {
        IRBuilder { builder }
    }

    pub fn set_position(&self, block: LLVMBasicBlockRef, instr: LLVMValueRef) {
        unsafe { LLVMPositionBuilder(self.builder, block, instr) }
    }

    pub fn set_position_end(&self, block: LLVMBasicBlockRef) {
        unsafe { LLVMPositionBuilderAtEnd(self.builder, block) }
    }

    pub fn create_alloc(&self, ty: &Type) -> Value {
        let value = unsafe {
            LLVMBuildAlloca(self.builder, ty.get_type(), NULL_STR)
        };
        Value::Empty
    }
}
