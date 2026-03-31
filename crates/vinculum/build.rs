use std::fs;
use std::path::Path;

mod build_scripts;

use build_scripts::build::{compiler, config, linker, validator};
use build_scripts::codegen::{dispatch, functions as codegen_functions};
use build_scripts::config::parser;

fn main() {
    let config = config::load_config();

    let functions_path = Path::new(&config.functions_file);
    validator::validate_functions_file(functions_path);

    let functions = parser::parse_haskell_functions(functions_path);

    codegen_functions::generate_functions(&functions);

    let haskell_dir = Path::new("build_scripts/haskell");
    let c_dir = Path::new("build_scripts/c");
    let user_functions_path = Path::new(&config.functions_file);

    dispatch::generate_haskell_dispatch(&functions, haskell_dir);

    let lib_path = Path::new(&config.lib_dir);
    fs::create_dir_all(lib_path).expect("Failed to create Haskell library output directory");

    validator::validate_library_dir(lib_path);

    compiler::compile_haskell_library(
        haskell_dir,
        c_dir,
        user_functions_path,
        lib_path,
        &config.lib_file,
    );

    validator::validate_main_library(&config);
    validator::warn_if_rts_missing(&config);

    compiler::copy_rts_library(&config.rts_dir, &config.rts_lib, lib_path);

    linker::emit_link_instructions(&config);
    linker::emit_rerun_instructions(&config.functions_file);

    println!(
        "cargo:rerun-if-changed={}",
        haskell_dir.join("Runtime.hs").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        haskell_dir.join("Codec.hs").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        haskell_dir.join("Dispatch.hs").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        Path::new(&config.functions_file).display()
    );

    for function in &functions {
        println!(
            "cargo:warning=Registered Haskell function: {}",
            function.name
        );
    }
}
