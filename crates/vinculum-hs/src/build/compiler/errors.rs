use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum CompilerError {
    #[error("cabal executable not found in PATH")]
    CabalNotFound,
    #[error("PATH environment variable not set")]
    PathNotSet,
    #[error("failed to run cabal build for target '{target}': {reason}")]
    CabalBuildFailed { target: String, reason: String },
    #[error("unsupported target OS for dynamic libraries")]
    UnsupportedOS,
    #[error("library '{library}' not found in {}", path.display())]
    LibraryNotFound { library: String, path: PathBuf },
    #[error("failed to resolve target directory: {reason}")]
    TargetDirResolutionFailed { reason: String },
    #[error("invalid cabal file path: {}", path.display())]
    InvalidCabalPath { path: PathBuf },
    #[error("failed to create directory '{}': {reason}", path.display())]
    DirectoryCreationFailed { path: PathBuf, reason: String },
    #[error("failed to copy file from '{}' to '{}': {reason}", src.display(), dst.display())]
    FileCopyFailed {
        src: PathBuf,
        dst: PathBuf,
        reason: String,
    },
    #[error("failed to read cargo metadata: {reason}")]
    CargoMetadataReadFailed { reason: String },
    #[error("no root package found in cargo metadata")]
    NoRootPackage,
    #[error("failed to get manifest directory from: {}", path.display())]
    ManifestDirResolutionFailed { path: PathBuf },
    #[error(
        "invalid config: define ALL of `cabal_file`, `exports_dir`, `foreign_library`, or NONE to use fallbacks"
    )]
    InvalidConfigPartial,
    #[error("cabal file does not exist: {}", path.display())]
    CabalFileNotFound { path: PathBuf },
    #[error("exports directory does not exist: {}", path.display())]
    ExportsDirNotFound { path: PathBuf },
    #[error("fallback cabal file does not exist: {}", path.display())]
    FallbackCabalNotFound { path: PathBuf },
    #[error("fallback exports directory does not exist: {}", path.display())]
    FallbackExportsDirNotFound { path: PathBuf },
    #[error("failed to read file '{}': {reason}", path.display())]
    FileReadFailed { path: PathBuf, reason: String },
    #[error("failed to write file '{}': {reason}", path.display())]
    FileWriteFailed { path: PathBuf, reason: String },
    #[error("failed to read directory '{}': {reason}", path.display())]
    DirectoryReadFailed { path: PathBuf, reason: String },
    #[error("invalid UTF-8 in path: {}", path.display())]
    InvalidUtf8Path { path: PathBuf },
    #[error("failed to find library recursive in {}", path.display())]
    LibrarySearchFailed { path: PathBuf },
    #[error("haskell compilation stopped due to previous errors")]
    HaskellCompilationStopped,
    #[error("unknown compiler error: {message}")]
    Unknown { message: String },
}
