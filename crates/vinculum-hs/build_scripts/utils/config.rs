use std::env;
use std::fs;
use std::path::Path;

use super::errors::ConfigLoadError;
use crate::build_scripts::build::compiler::{find_rts_dir, find_rts_lib};

#[derive(Clone)]
pub struct BuildConfig {
    pub lib_dir: String,
    pub lib_file: String,
    pub rts_lib: String,
    pub rts_dir: String,
    pub functions_dir: String,
}

fn read_haskell_dir_from_cargo_toml(manifest_path: &Path, bin_name: &str) -> Option<String> {
    let content = fs::read_to_string(manifest_path).ok()?;

    let section_header = format!("[package.metadata.vinculum.examples.{}]", bin_name);
    if let Some(metadata_start) = content.find(&section_header) {
        let metadata_section = &content[metadata_start..];

        for line in metadata_section.lines() {
            if line.starts_with("haskell_directory")
                && let Some(start) = line.find('"')
                && let Some(end) = line.rfind('"')
                && end > start
            {
                let value = &line[start + 1..end];
                return Some(value.to_string());
            }

            if line.starts_with('[') && !line.starts_with(&section_header) {
                break;
            }
        }
    }

    None
}

fn find_haskell_dir(workspace_root: &str) -> Result<String, ConfigLoadError> {
    if let Ok(dir) = env::var("HASKELL_DIR") {
        return Ok(dir);
    }

    let manifest_path = Path::new(workspace_root)
        .join("crates")
        .join("vinculum-hs")
        .join("Cargo.toml");

    if let Ok(bin_name) = env::var("CARGO_BIN_NAME")
        && let Some(dir) = read_haskell_dir_from_cargo_toml(&manifest_path, &bin_name)
    {
        let full_path = Path::new(workspace_root).join(&dir);
        if full_path.exists() && full_path.is_dir() {
            return Ok(full_path.to_string_lossy().to_string());
        }
    }

    let fallback = Path::new(workspace_root)
        .join("crates")
        .join("vinculum-hs")
        .join("haskell_fallback");
    if fallback.exists() && fallback.is_dir() {
        return Ok(fallback.to_string_lossy().to_string());
    }

    Err(ConfigLoadError::HaskellDirNotFound)
}

pub fn load_config() -> Result<BuildConfig, ConfigLoadError> {
    let rts_dir = find_rts_dir();
    if rts_dir.trim().is_empty() {
        return Err(ConfigLoadError::RtsDirNotFound);
    }

    let rts_lib = find_rts_lib(&rts_dir);
    if rts_lib.trim().is_empty() {
        return Err(ConfigLoadError::RtsLibNotFound);
    }

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());

    let workspace_root = Path::new(&manifest_dir)
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| ".".to_string());

    let lib_dir = format!("{}/target/haskell", workspace_root);
    let functions_dir = find_haskell_dir(&workspace_root)?;

    Ok(BuildConfig {
        lib_dir,
        lib_file: "userLib".to_string(),
        rts_lib,
        rts_dir,
        functions_dir,
    })
}
