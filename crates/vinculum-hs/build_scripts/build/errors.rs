use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum BuildValidationError {
    LibraryDirDoesNotExist(PathBuf),
    LibraryDirIsNotDirectory(PathBuf),
    MainLibraryNotFound(PathBuf),
    MainLibraryIsNotFile(PathBuf),
}

impl fmt::Display for BuildValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildValidationError::LibraryDirDoesNotExist(path) => {
                write!(
                    f,
                    "invalid configuration: HASKELL_LIB_DIR does not exist: {}",
                    path.display()
                )
            }
            BuildValidationError::LibraryDirIsNotDirectory(path) => {
                write!(
                    f,
                    "invalid configuration: HASKELL_LIB_DIR is not a directory: {}",
                    path.display()
                )
            }
            BuildValidationError::MainLibraryNotFound(path) => {
                write!(
                    f,
                    "linking error: Haskell library not found at expected path: {}",
                    path.display()
                )
            }
            BuildValidationError::MainLibraryIsNotFile(path) => {
                write!(
                    f,
                    "linking error: expected Haskell library is not a file: {}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for BuildValidationError {}
