use crate::{
    Context, Function, LLVMAppendBasicBlockInContext, LLVMBasicBlockRef, LLVMCreateBuilder,
    LLVMCreateBuilderInContext, LLVMPositionBuilderAtEnd,
};

pub struct BasicBlock {
    basic_block: LLVMBasicBlockRef,
}

impl BasicBlock {
    pub fn create(name: &str, context: &Context, function: &Function) -> Self {
        let basic_block = unsafe {
            LLVMAppendBasicBlockInContext(
                context.context(),
                function.function(),
                &name.as_bytes()[0] as *const u8,
            )
        };
        BasicBlock { basic_block }
    }

    pub fn basic_block(&self) -> LLVMBasicBlockRef {
        self.basic_block
    }
}

pub struct Builder {
    builder: LLVMBasicBlockRef,
}

impl Builder {
    pub fn create(context: &Context) -> Self {
        let builder = unsafe { LLVMCreateBuilderInContext(context.context()) };
        Builder { builder }
    }

    pub fn position_at_end(&self, basic_block: &BasicBlock) {
        unsafe { LLVMPositionBuilderAtEnd(self.builder, basic_block.basic_block()) }
    }
}
