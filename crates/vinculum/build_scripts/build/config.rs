use crate::build_scripts::build::compiler::{find_rts_dir, find_rts_lib};
use std::env;
use std::path::Path;

pub struct BuildConfig {
    pub lib_dir: String,
    pub lib_file: String,
    pub rts_lib: String,
    pub rts_dir: String,
    pub functions_dir: String,
}

fn find_haskell_dir(workspace_root: &str) -> String {
    if let Ok(dir) = env::var("HASKELL_DIR") {
        return dir;
    }

    let examples_dir = Path::new(workspace_root).join("examples");
    if examples_dir.exists()
        && let Ok(entries) = std::fs::read_dir(&examples_dir)
    {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir()
                && let Ok(subentries) = std::fs::read_dir(&path)
            {
                for subentry in subentries.flatten() {
                    let subpath = subentry.path();
                    if subpath.is_file()
                        && subpath.extension().and_then(|s| s.to_str()) == Some("hs")
                    {
                        return path.to_string_lossy().to_string();
                    }
                }
            }
        }
    }

    format!("{}/examples/add", workspace_root)
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
    let functions_dir = find_haskell_dir(&workspace_root);

    BuildConfig {
        lib_dir,
        lib_file: "userLib".to_string(),
        rts_lib,
        rts_dir,
        functions_dir,
    }
}
