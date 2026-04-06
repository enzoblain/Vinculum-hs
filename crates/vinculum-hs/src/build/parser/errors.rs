use std::io;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(crate) enum ParseError {
    #[error("failed to read file '{}': {source}", path.display())]
    ReadFile { path: PathBuf, source: io::Error },
    #[error("empty function signature")]
    EmptySignature,
    #[error(
        "invalid function name `{name}` in signature `{signature}` (not a valid Rust identifier)"
    )]
    InvalidFunctionName { name: String, signature: String },
    #[error("name `{name}` is a reserved Rust keyword")]
    ReservedRustKeyword { name: String },
    #[error("unsupported type `{0}`")]
    UnsupportedHaskellType(String),
    #[error("missing type annotation in signature `{signature}` (expected `::`)")]
    MissingHaskellTypeAnnotation { signature: String },
    #[error("missing return type in signature `{signature}`")]
    MissingReturnHaskellType { signature: String },
    #[error("argument count mismatch in `{signature}`: expected {expected}, found {found}")]
    ArgumentCountMismatch {
        expected: usize,
        found: usize,
        signature: String,
    },
}
