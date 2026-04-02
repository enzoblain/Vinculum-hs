use std::path::Path;

use super::errors::BuildValidationError;
use crate::build_scripts::utils::config::BuildConfig;

pub(crate) fn validate_library_dir(path: &Path) -> Result<(), BuildValidationError> {
    if !path.exists() {
        return Err(BuildValidationError::LibraryDirDoesNotExist(
            path.to_path_buf(),
        ));
    }

    if !path.is_dir() {
        return Err(BuildValidationError::LibraryDirIsNotDirectory(
            path.to_path_buf(),
        ));
    }

    Ok(())
}

pub(crate) fn shared_library_extension() -> &'static str {
    if cfg!(target_os = "windows") {
        "dll"
    } else if cfg!(target_os = "macos") {
        "dylib"
    } else {
        "so"
    }
}

pub(crate) fn library_filename(name: &str, ext: &str) -> String {
    if cfg!(target_os = "windows") {
        format!("{}.{}", name, ext)
    } else {
        format!("lib{}.{}", name, ext)
    }
}

pub(crate) fn validate_main_library(config: &BuildConfig) -> Result<(), BuildValidationError> {
    let lib_path = Path::new(&config.lib_dir);
    let ext = shared_library_extension();
    let lib_filename = library_filename(&config.lib_file, ext);
    let full_lib_path = lib_path.join(&lib_filename);

    if !full_lib_path.exists() {
        return Err(BuildValidationError::MainLibraryNotFound(full_lib_path));
    }

    if !full_lib_path.is_file() {
        return Err(BuildValidationError::MainLibraryIsNotFile(full_lib_path));
    }

    Ok(())
}

pub(crate) fn warn_if_rts_missing(config: &BuildConfig) {
    let lib_path = Path::new(&config.lib_dir);
    let ext = shared_library_extension();
    let rts_filename = library_filename(&config.rts_lib, ext);
    let rts_path = lib_path.join(&rts_filename);

    if !rts_path.exists() {
        println!(
            "cargo:warning=Haskell RTS library '{}' not found in '{}'. Ensure the GHC runtime library path is correctly configured.",
            rts_filename, config.lib_dir
        );
    }
}
