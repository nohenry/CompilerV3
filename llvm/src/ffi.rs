use std::os::raw::c_char;

pub enum LLVMContext {}
pub enum LLVMModule {}
pub enum LLVMType {}
pub enum LLVMValue {}
pub enum LLVMFunctionType {}
pub enum LLVMBasicBlock {}
pub enum LLVMBuilder {}
pub enum LLVMTargetMachine {}
pub enum LLVMTarget {}

pub type LLVMContextRef = *mut LLVMContext;
pub type LLVMModuleRef = *mut LLVMContextRef;
pub type LLVMTypeRef = *mut LLVMType;
pub type LLVMValueRef = *mut LLVMValue;
pub type LLVMFunctionTypeRef = *mut LLVMFunctionType;
pub type LLVMBasicBlockRef = *mut LLVMBasicBlock;
pub type LLVMBuilderRef = *mut LLVMBuilder;
pub type LLVMTargetMachineRef = *mut LLVMTargetMachine;
pub type LLVMTargetRef = *mut LLVMTarget;

/// LLVMRustFileType
#[derive(Copy, Clone)]
#[repr(C)]
pub enum FileType {
    AssemblyFile,
    ObjectFile,
}

#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub enum CodeGenOptLevel {
    None,
    Less,
    Default,
    Aggressive,
}

#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub enum RelocModel {
    Default,
    Static,
    PIC,
    DynamicNoPic,
    ROPI,
    RWPI,
    ROPI_RWPI,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum CodeModel {
    Default,
    JITDefault,
    Tiny,
    Small,
    Kernel,
    Medium,
    Large,
}

#[link(name = "LLVM-C", kind = "dylib")]
// #[link(name = "LLVMCore", kind = "static")]
extern "C" {
    // Context
    pub fn LLVMContextCreate() -> LLVMContextRef;

    // Modules
    pub fn LLVMModuleCreateWithName(name: *const u8) -> LLVMModuleRef;
    pub fn LLVMModuleCreateWithNameInContext(
        name: *const u8,
        context: LLVMContextRef,
    ) -> LLVMModuleRef;
    pub fn LLVMWriteBitcodeToFile(module: LLVMModuleRef, path: *const u8) -> i32;
    pub fn LLVMAddFunction(
        module: LLVMModuleRef,
        name: *const u8,
        function_type: LLVMFunctionTypeRef,
    ) -> LLVMValueRef;
    pub fn LLVMTargetMachineEmitToFile(
        target_machine: LLVMTargetMachineRef,
        module: LLVMModuleRef,
        filename: *const c_char,
        file_type: FileType,
        error_message: *mut *const c_char,
    ) -> bool;

    // Targets

    pub fn LLVMGetDefaultTargetTriple() -> *const c_char;
    pub fn LLVMGetTargetFromTriple(
        triple: *const c_char,
        target: *mut LLVMTargetRef,
        error_message: *mut *const c_char,
    ) -> bool;
    pub fn LLVMCreateTargetMachine(
        target: LLVMTargetRef,
        triple: *const c_char,
        cpu: *const c_char,
        features: *const c_char,
        level: CodeGenOptLevel,
        reloc: RelocModel,
        code_model: CodeModel,
    ) -> LLVMTargetMachineRef;
    pub fn LLVMGetFirstTarget() -> LLVMTargetRef;

    // Types
    pub fn LLVMInt32Type() -> LLVMTypeRef;
    pub fn LLVMFunctionType(
        return_type: LLVMTypeRef,
        param_types: *const LLVMTypeRef,
        param_count: u32,
        is_var_arg: bool,
    ) -> LLVMFunctionTypeRef;

    // Blocks and builders
    pub fn LLVMAppendBasicBlockInContext(
        context: LLVMContextRef,
        func: LLVMValueRef,
        name: *const u8,
    ) -> LLVMBasicBlockRef;
    pub fn LLVMAppendBasicBlock(func: LLVMValueRef, name: *const u8) -> LLVMBasicBlockRef;
    pub fn LLVMCreateBuilder() -> LLVMBuilderRef;
    pub fn LLVMCreateBuilderInContext(context: LLVMContextRef) -> LLVMBuilderRef;
    pub fn LLVMPositionBuilderAtEnd(builder: LLVMBuilderRef, block: LLVMBasicBlockRef);

}

#[link(name = "wrappers", kind = "static")]
extern "C" {
    pub fn InitializeAllTargets();
    pub fn InitializeAllTargetMCs();
    pub fn InitializeAllAsmPrinters();
    pub fn InitializeAllAsmParsers();
    pub fn FindTargets();
}
