use std::slice;

use crate::ffi::{call_haskell_function, free_haskell_buffer};

include!(concat!(env!("OUT_DIR"), "/generated_functions.rs"));

pub(super) enum Value {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),

    Word8(u8),
    Word16(u16),
    Word32(u32),
    Word64(u64),

    Float32(f32),
    Float64(f64),

    Bool(bool),
    Char(char),
    String(String),
    Bytes(Vec<u8>),
}

impl Value {
    #[allow(dead_code)]
    fn into_int8(self) -> i8 {
        match self {
            Value::Int8(value) => value,
            _ => panic!("Expected Value::Int8"),
        }
    }

    #[allow(dead_code)]
    fn into_int16(self) -> i16 {
        match self {
            Value::Int16(value) => value,
            _ => panic!("Expected Value::Int16"),
        }
    }

    #[allow(dead_code)]
    fn into_int32(self) -> i32 {
        match self {
            Value::Int32(value) => value,
            _ => panic!("Expected Value::Int32"),
        }
    }

    #[allow(dead_code)]
    fn into_int64(self) -> i64 {
        match self {
            Value::Int64(value) => value,
            _ => panic!("Expected Value::Int64"),
        }
    }

    #[allow(dead_code)]
    fn into_word8(self) -> u8 {
        match self {
            Value::Word8(value) => value,
            _ => panic!("Expected Value::Word8"),
        }
    }

    #[allow(dead_code)]
    fn into_word16(self) -> u16 {
        match self {
            Value::Word16(value) => value,
            _ => panic!("Expected Value::Word16"),
        }
    }

    #[allow(dead_code)]
    fn into_word32(self) -> u32 {
        match self {
            Value::Word32(value) => value,
            _ => panic!("Expected Value::Word32"),
        }
    }

    #[allow(dead_code)]
    fn into_word64(self) -> u64 {
        match self {
            Value::Word64(value) => value,
            _ => panic!("Expected Value::Word64"),
        }
    }

    #[allow(dead_code)]
    fn into_float32(self) -> f32 {
        match self {
            Value::Float32(value) => value,
            _ => panic!("Expected Value::Float32"),
        }
    }

    #[allow(dead_code)]
    fn into_float64(self) -> f64 {
        match self {
            Value::Float64(value) => value,
            _ => panic!("Expected Value::Float64"),
        }
    }

    #[allow(dead_code)]
    fn into_bool(self) -> bool {
        match self {
            Value::Bool(value) => value,
            _ => panic!("Expected Value::Bool"),
        }
    }

    #[allow(dead_code)]
    fn into_char(self) -> char {
        match self {
            Value::Char(value) => value,
            _ => panic!("Expected Value::Char"),
        }
    }

    #[allow(dead_code)]
    fn into_string(self) -> String {
        match self {
            Value::String(value) => value,
            _ => panic!("Expected Value::String"),
        }
    }

    #[allow(dead_code)]
    fn into_bytes(self) -> Vec<u8> {
        match self {
            Value::Bytes(value) => value,
            _ => panic!("Expected Value::Bytes"),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        match self {
            Value::Int8(x) => {
                buf.push(0);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Int16(x) => {
                buf.push(1);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Int32(x) => {
                buf.push(2);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Int64(x) => {
                buf.push(3);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Word8(x) => {
                buf.push(4);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Word16(x) => {
                buf.push(5);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Word32(x) => {
                buf.push(6);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Word64(x) => {
                buf.push(7);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Float32(x) => {
                buf.push(8);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Float64(x) => {
                buf.push(9);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Bool(b) => {
                buf.push(10);
                buf.push(*b as u8);
            }
            Value::Char(c) => {
                buf.push(11);
                let code = *c as u32;
                buf.extend_from_slice(&code.to_le_bytes());
            }
            Value::String(s) => {
                buf.push(12);
                let bytes = s.as_bytes();
                buf.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
                buf.extend_from_slice(bytes);
            }
            Value::Bytes(b) => {
                buf.push(13);
                buf.extend_from_slice(&(b.len() as u64).to_le_bytes());
                buf.extend_from_slice(b);
            }
        }

        buf
    }

    #[allow(dead_code)]
    fn encode_slice(args: &[Value]) -> Vec<u8> {
        let mut buf = Vec::new();

        for arg in args {
            buf.extend_from_slice(&arg.to_bytes());
        }

        buf
    }

    fn from_bytes(bytes: &[u8]) -> Value {
        let tag = bytes[0];

        match tag {
            0 => {
                let mut arr = [0u8; 1];
                arr.copy_from_slice(&bytes[1..2]);
                Value::Int8(i8::from_le_bytes(arr))
            }
            1 => {
                let mut arr = [0u8; 2];
                arr.copy_from_slice(&bytes[1..3]);
                Value::Int16(i16::from_le_bytes(arr))
            }
            2 => {
                let mut arr = [0u8; 4];
                arr.copy_from_slice(&bytes[1..5]);
                Value::Int32(i32::from_le_bytes(arr))
            }
            3 => {
                let mut arr = [0u8; 8];
                arr.copy_from_slice(&bytes[1..9]);
                Value::Int64(i64::from_le_bytes(arr))
            }
            4 => {
                let mut arr = [0u8; 1];
                arr.copy_from_slice(&bytes[1..2]);
                Value::Word8(u8::from_le_bytes(arr))
            }
            5 => {
                let mut arr = [0u8; 2];
                arr.copy_from_slice(&bytes[1..3]);
                Value::Word16(u16::from_le_bytes(arr))
            }
            6 => {
                let mut arr = [0u8; 4];
                arr.copy_from_slice(&bytes[1..5]);
                Value::Word32(u32::from_le_bytes(arr))
            }
            7 => {
                let mut arr = [0u8; 8];
                arr.copy_from_slice(&bytes[1..9]);
                Value::Word64(u64::from_le_bytes(arr))
            }
            8 => {
                let mut arr = [0u8; 4];
                arr.copy_from_slice(&bytes[1..5]);
                Value::Float32(f32::from_le_bytes(arr))
            }
            9 => {
                let mut arr = [0u8; 8];
                arr.copy_from_slice(&bytes[1..9]);
                Value::Float64(f64::from_le_bytes(arr))
            }
            10 => Value::Bool(bytes[1] != 0),
            11 => {
                let mut arr = [0u8; 4];
                arr.copy_from_slice(&bytes[1..5]);
                let code = u32::from_le_bytes(arr);
                Value::Char(char::from_u32(code).expect("Invalid char code"))
            }
            12 => {
                let mut len_bytes = [0u8; 8];
                len_bytes.copy_from_slice(&bytes[1..9]);
                let len = u64::from_le_bytes(len_bytes) as usize;

                let s = String::from_utf8(bytes[9..9 + len].to_vec()).expect("Invalid UTF-8");
                Value::String(s)
            }
            13 => {
                let mut len_bytes = [0u8; 8];
                len_bytes.copy_from_slice(&bytes[1..9]);
                let len = u64::from_le_bytes(len_bytes) as usize;

                Value::Bytes(bytes[9..9 + len].to_vec())
            }
            _ => panic!("Invalid type tag"),
        }
    }
}

pub(super) fn call_haskell_typed(name: &str, args: &[Value]) -> Value {
    let input = Value::encode_slice(args);

    let mut out_ptr: *mut u8 = std::ptr::null_mut();
    let mut out_len: usize = 0;

    unsafe {
        call_haskell_function(
            name.as_ptr(),
            name.len(),
            input.as_ptr(),
            input.len(),
            &mut out_ptr as *mut *mut u8,
            &mut out_len as *mut usize,
        );
    }

    if out_ptr.is_null() {
        panic!("Haskell returned a null buffer");
    }

    let bytes = unsafe { slice::from_raw_parts(out_ptr, out_len) };
    let result = Value::from_bytes(bytes);

    unsafe {
        free_haskell_buffer(out_ptr);
    }

    result
}
