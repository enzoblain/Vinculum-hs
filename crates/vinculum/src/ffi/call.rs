use std::slice;

use crate::ffi::bindings::{call_haskell_function, free_haskell_buffer};
use crate::ffi::errors::FfiError;
use crate::ffi::value::Value;

pub(crate) fn call_haskell_typed(name: &str, args: &[Value]) -> Value {
    call_haskell_typed_checked(name, args)
        .expect("internal FFI error while calling Haskell function")
}

pub(crate) fn call_haskell_typed_checked(name: &str, args: &[Value]) -> Result<Value, FfiError> {
    let input = Value::encode_slice(args);

    let mut out_ptr: *mut u8 = std::ptr::null_mut();
    let mut out_len: usize = 0;

    unsafe {
        call_haskell_function(
            name.as_ptr(),
            name.len(),
            input.as_ptr(),
            input.len(),
            &mut out_ptr,
            &mut out_len,
        );
    }

    if out_ptr.is_null() {
        return Err(FfiError::NullPointer);
    }

    let bytes = unsafe { slice::from_raw_parts(out_ptr, out_len) };
    let result = Value::from_bytes(bytes);

    unsafe {
        free_haskell_buffer(out_ptr);
    }

    Ok(result)
}
