use std::ffi::c_char;
use std::os::raw::c_int;
use std::ptr;

use crate::ffi::{haskell_exit, haskell_init};

pub fn init() {
    unsafe {
        let argc: c_int = 0;
        let argv: *mut *mut c_char = ptr::null_mut();

        haskell_init(argc, argv);
    }
}

pub fn shutdown() {
    unsafe {
        haskell_exit();
    }
}
