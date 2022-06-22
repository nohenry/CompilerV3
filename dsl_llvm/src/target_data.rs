use dsl_symbol::Type;
use llvm_sys::target::{LLVMPreferredAlignmentOfType, LLVMTargetDataRef};

pub struct TargetData(LLVMTargetDataRef);

impl TargetData {
    pub fn new(target_data: LLVMTargetDataRef) -> TargetData {
        TargetData(target_data)
    }

    pub fn get_preferred_align(&self, ty: &Type) -> u32 {
        unsafe { LLVMPreferredAlignmentOfType(self.0, ty.get_type()) }
    }
}
