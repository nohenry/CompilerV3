#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[macro_export]
macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const u8
    };
}

pub mod direct_bindings;
pub use direct_bindings::*;

pub mod module;
pub use module::*;

pub mod context;
pub use context::*;

pub mod builder;
pub use builder::*;

pub mod values;
pub use values::*;