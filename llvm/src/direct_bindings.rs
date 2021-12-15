
pub type LLVMContextRef = *mut ();
pub type LLVMModuleRef = *mut ();
pub type LLVMTypeRef = *mut ();
pub type LLVMValueRef = *mut ();
pub type LLVMFunctionTypeRef = *mut ();
pub type LLVMBasicBlockRef = *mut ();
pub type LLVMBuilderRef= *mut ();

#[link(name = "LLVM-C", kind="dylib")]
extern "C" {
    pub fn LLVMContextCreate() -> LLVMContextRef;
    pub fn LLVMModuleCreateWithName(name: *const u8) -> LLVMModuleRef;
    pub fn LLVMInt32Type() -> LLVMTypeRef;
    pub fn LLVMFunctionType(return_type: LLVMTypeRef, param_types: *const LLVMTypeRef, param_count: u32, is_var_arg: bool) -> LLVMFunctionTypeRef;
    pub fn LLVMAddFunction(module: LLVMModuleRef, name: *const u8, function_type: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMWriteBitcodeToFile(module: LLVMModuleRef, path: *const u8) -> i32;
    pub fn LLVMModuleCreateWithNameInContext(name: *const u8, context: LLVMContextRef) -> LLVMModuleRef;
    pub fn LLVMAppendBasicBlockInContext(context: LLVMContextRef, func: LLVMValueRef, name: *const u8) -> LLVMBasicBlockRef;
    pub fn LLVMAppendBasicBlock(func: LLVMValueRef, name: *const u8) -> LLVMBasicBlockRef;
    pub fn LLVMCreateBuilder() -> LLVMBuilderRef;
    pub fn LLVMCreateBuilderInContext(context: LLVMContextRef) -> LLVMBuilderRef;
    pub fn LLVMPositionBuilderAtEnd(builder: LLVMBuilderRef, block: LLVMBasicBlockRef);
}
