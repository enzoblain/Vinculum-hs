use crate::build_scripts::build::compiler::{find_rts_dir, find_rts_lib};
use std::env;
use std::path::Path;

pub struct BuildConfig {
    pub lib_dir: String,
    pub lib_file: String,
    pub rts_lib: String,
    pub rts_dir: String,
    pub functions_file: String,
}

fn find_haskell_file(workspace_root: &str) -> String {
    if let Ok(file) = env::var("HASKELL_FILE") {
        return file;
    }

    let examples_dir = Path::new(workspace_root).join("examples");
    if examples_dir.exists() {
        for entry in std::fs::read_dir(&examples_dir)
            .unwrap_or_else(|_| std::fs::read_dir(workspace_root).unwrap())
            .flatten()
        {
            let path = entry.path();
            if path.is_dir() {
                let script_file = path.join("Script.hs");
                if script_file.exists()
                    && let Some(s) = script_file.to_str()
                {
                    return s.to_string();
                }
            }
        }
    }

    format!("{}/Script.hs", workspace_root)
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
    let functions_file = find_haskell_file(&workspace_root);

    BuildConfig {
        lib_dir,
        lib_file: "userLib".to_string(),
        rts_lib,
        rts_dir,
        functions_file,
    }
}
