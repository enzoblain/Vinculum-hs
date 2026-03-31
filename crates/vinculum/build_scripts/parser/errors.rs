use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum ConfigError {
    NoFunctionsDefined,
    EmptyFunctionName,
    EmptyArgumentName { function: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::NoFunctionsDefined => {
                write!(f, "configuration error: no functions defined")
            }
            ConfigError::EmptyFunctionName => {
                write!(f, "configuration error: function name cannot be empty")
            }
            ConfigError::EmptyArgumentName { function } => {
                write!(
                    f,
                    "configuration error: argument name cannot be empty in function '{}'",
                    function
                )
            }
        }
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug)]
pub(crate) enum ParseError {
    ReadFile {
        path: PathBuf,
        source: std::io::Error,
    },
    InvalidSignature(String),
    UnknownType(String),
    Validation(ConfigError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::ReadFile { path, source } => {
                write!(
                    f,
                    "failed to read Haskell file '{}': {}",
                    path.display(),
                    source
                )
            }
            ParseError::InvalidSignature(signature) => {
                write!(f, "invalid Haskell signature: {}", signature)
            }
            ParseError::UnknownType(ty) => {
                write!(f, "unknown Haskell type: {}", ty)
            }
            ParseError::Validation(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for ParseError {}
