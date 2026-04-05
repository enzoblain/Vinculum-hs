pub mod build_scripts;
pub mod ffi;
pub mod runtime;

pub use vinculum_hs_macros::main;
pub use crate::build_scripts::build::core::build;
