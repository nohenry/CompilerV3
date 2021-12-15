use crate::{
    BasicBlock, Builder, Function, LLVMAppendBasicBlock, LLVMContextCreate, LLVMContextRef, Module,
};

pub struct Context {
    context: LLVMContextRef,
}

impl Context {
    pub fn create() -> Self {
        let context = unsafe { LLVMContextCreate() };
        Context { context }
    }

    pub fn create_module(&self, name: &str) -> Module {
        Module::create(name, self)
    }

    pub fn create_builder(&self) -> Builder {
        Builder::create(self)
    }

    pub fn context(&self) -> LLVMContextRef {
        self.context
    }

    pub fn append_basic_block(function: &Function, name: &str) {
        unsafe {
            LLVMAppendBasicBlock(function.function(), &name.as_bytes()[0] as *const u8);
        }
    }
}
