use std::fs;
use std::path::Path;

mod build_scripts;

use build_scripts::build::{compiler, linker, validator};
use build_scripts::codegen::{generate_functions_with_modules, generate_haskell_dispatch};
use build_scripts::utils::config::load_config;
use build_scripts::utils::helpers::{
    collect_file_modules, emit_rerun_if_changed, generate_user_functions_module,
    log_registered_functions,
};

fn main() {
    let config =
        load_config().unwrap_or_else(|e| panic!("Failed to load build configuration: {e}"));

    let haskell_dir = Path::new(&config.functions_dir);

    assert!(
        haskell_dir.exists(),
        "Invalid configuration: HASKELL_DIR does not exist: {}",
        haskell_dir.display()
    );
    assert!(
        haskell_dir.is_dir(),
        "Invalid configuration: HASKELL_DIR is not a directory: {}",
        haskell_dir.display()
    );

    let target_haskell_dir = Path::new(&config.lib_dir);
    fs::create_dir_all(target_haskell_dir)
        .unwrap_or_else(|e| panic!("Failed to create Haskell build dir: {e}"));

    let file_modules = collect_file_modules(haskell_dir, target_haskell_dir);

    let user_functions_content = generate_user_functions_module(&file_modules);
    let user_functions_path = target_haskell_dir.join("UserFunctions.hs");
    fs::write(&user_functions_path, user_functions_content)
        .unwrap_or_else(|e| panic!("Failed to write '{}': {e}", user_functions_path.display()));

    generate_functions_with_modules(&file_modules);

    let ffi_dir = Path::new("build_scripts/ffi");
    generate_haskell_dispatch(&file_modules, ffi_dir)
        .unwrap_or_else(|e| panic!("Failed to generate Haskell dispatch: {e}"));

    let lib_path = Path::new(&config.lib_dir);
    fs::create_dir_all(lib_path)
        .unwrap_or_else(|e| panic!("Failed to create Haskell library output directory: {e}"));

    validator::validate_library_dir(lib_path).unwrap_or_else(|e| panic!("{e}"));
    compiler::compile_haskell_library(ffi_dir, &user_functions_path, lib_path, &config.lib_file);
    validator::validate_main_library(&config).unwrap_or_else(|e| panic!("{e}"));
    validator::warn_if_rts_missing(&config);
    compiler::copy_rts_library(&config.rts_dir, &config.rts_lib, lib_path);
    linker::emit_link_instructions(&config)
        .unwrap_or_else(|e| panic!("Failed to emit link instructions: {e}"));
    linker::emit_rerun_instructions(&config.functions_dir);

    emit_rerun_if_changed(ffi_dir, &config.functions_dir);
    log_registered_functions(&file_modules);
}
