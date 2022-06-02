use crate::LLVMValueRef;

pub struct Function {
    function: LLVMValueRef,
}

impl Function {
    pub fn function(&self) -> LLVMValueRef {
        self.function
    }
}
