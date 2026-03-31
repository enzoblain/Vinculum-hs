use std::fmt;

#[derive(Debug)]
pub enum FfiError {
    NullPointer,
    InvalidTag(u8),
    UnexpectedEof,
    InvalidUtf8,
    InvalidChar(u32),
    DecodeError,
}

impl fmt::Display for FfiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FfiError::NullPointer => write!(f, "Received null pointer from Haskell"),
            FfiError::InvalidTag(tag) => write!(f, "Invalid type tag: {}", tag),
            FfiError::UnexpectedEof => write!(f, "Unexpected end of input"),
            FfiError::InvalidUtf8 => write!(f, "Invalid UTF-8 string"),
            FfiError::InvalidChar(c) => write!(f, "Invalid char code: {}", c),
            FfiError::DecodeError => write!(f, "Failed to decode value"),
        }
    }
}

impl std::error::Error for FfiError {}
