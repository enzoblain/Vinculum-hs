#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum ConfigLoadError {
    HaskellDirNotFound,
    RtsDirNotFound,
    RtsLibNotFound,
}

impl std::fmt::Display for ConfigLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigLoadError::HaskellDirNotFound => {
                write!(f, "failed to locate a Haskell functions directory")
            }
            ConfigLoadError::RtsDirNotFound => {
                write!(f, "failed to locate the GHC RTS directory")
            }
            ConfigLoadError::RtsLibNotFound => {
                write!(f, "failed to locate the GHC RTS library")
            }
        }
    }
}

impl std::error::Error for ConfigLoadError {}
