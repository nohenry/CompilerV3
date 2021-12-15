use crate::context::Context;
use crate::{LLVMModuleCreateWithNameInContext, LLVMModuleRef};

pub struct Module {
    module: LLVMModuleRef,
}

impl Module {
    pub fn create(name: &str, context: &Context) -> Self {
        let module = unsafe {
            LLVMModuleCreateWithNameInContext(&name.as_bytes()[0] as *const u8, context.context())
        };
        Module {module}
    }
}
