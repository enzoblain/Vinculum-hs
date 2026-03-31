use crate::build_scripts::build::compiler::{find_rts_dir, find_rts_lib};
use std::env;

pub struct BuildConfig {
    pub lib_dir: String,
    pub lib_file: String,
    pub rts_lib: String,
    pub rts_dir: String,
    pub functions_file: String,
    pub haskell_dir: String,
    pub c_dir: String,
}

pub fn load_config() -> BuildConfig {
    let rts_dir = find_rts_dir();
    let rts_lib = find_rts_lib(&rts_dir);

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let workspace_root = std::path::Path::new(&manifest_dir)
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| ".".to_string());

    let lib_dir = format!("{}/target/haskell", workspace_root);

    BuildConfig {
        lib_dir,
        lib_file: env::var("HASKELL_LIB_FILE").unwrap_or_else(|_| "HSmylib".to_string()),
        rts_lib,
        rts_dir,
        functions_file: env::var("HASKELL_FILE")
            .unwrap_or_else(|_| format!("{}/examples/add/Script.hs", workspace_root)),
        haskell_dir: format!("{}/crates/vinculum/build_scripts/haskell", workspace_root),
        c_dir: format!("{}/crates/vinculum/build_scripts/c", workspace_root),
    }
}
