#![feature(extern_types)]
#![feature(crate_visibility_modifier)]
#![feature(iter_intersperse)]

// pub use llvm_sys::prelude::*;
// pub use llvm_sys::*;
// use llvm_sys::target::LLVM_InitializeAllTargets;

// pub fn bruh() {
//     unsafe {
//         LLVM_InitializeAllTargets();
//     }
// }

// pub mod direct_bindings;
// pub use direct_bindings::*;
// pub mod ffi;
// pub use ffi::*;

// pub mod module;
// pub use module::*;

// pub mod context;
// pub use context::*;

pub mod builder;
pub use builder::*;

pub mod target_data;

// pub mod values;
// pub use values::*;
