#[derive(Debug, thiserror::Error)]
pub enum FfiError {
    #[error("Received null pointer from Haskell")]
    NullPointer,
    #[error("Invalid type tag: {0}")]
    InvalidTag(u8),
    #[error("Unexpected end of input")]
    UnexpectedEof,
    #[error("Invalid char code: {0}")]
    InvalidChar(u32),
    #[error("Failed to decode value")]
    DecodeError,
}
