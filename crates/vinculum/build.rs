use std::fs;
use std::path::Path;

mod build_scripts;

use build_scripts::build::{compiler, config, linker, validator};
use build_scripts::codegen::{dispatch, functions as codegen_functions};
use build_scripts::config::parser;

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn main() {
    let config = config::load_config();

    let haskell_dir = Path::new(&config.functions_dir);

    if !haskell_dir.exists() {
        panic!(
            "Invalid configuration: HASKELL_DIR does not exist: {}",
            haskell_dir.display()
        );
    }

    if !haskell_dir.is_dir() {
        panic!(
            "Invalid configuration: HASKELL_DIR is not a directory: {}",
            haskell_dir.display()
        );
    }

    let mut file_modules = Vec::new();

    let target_haskell_dir = Path::new(&config.lib_dir);
    fs::create_dir_all(target_haskell_dir).expect("Failed to create haskell build dir");

    if let Ok(entries) = fs::read_dir(haskell_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file()
                && path.extension().and_then(|s| s.to_str()) == Some("hs")
                && let Some(file_name) = path.file_stem().and_then(|s| s.to_str())
            {
                let target_file = target_haskell_dir.join(format!("{}.hs", file_name));
                fs::copy(&path, &target_file).expect("Failed to copy .hs file");

                let file_functions = parser::parse_haskell_functions(&path);
                if !file_functions.is_empty() {
                    let module_name = capitalize_first(file_name);
                    file_modules.push((module_name.clone(), file_functions.clone()));
                }
            }
        }
    }

    let user_functions_content = if !file_modules.is_empty() {
        let mut exports = Vec::new();
        let mut imports = Vec::new();
        let mut wrappers = Vec::new();

        for (module_name, functions) in &file_modules {
            imports.push(format!(
                "import qualified {} as {}",
                module_name, module_name
            ));

            for function in functions {
                let qualified_name = format!("{}_{}", module_name.to_lowercase(), function.name);
                exports.push(qualified_name.clone());
                wrappers.push(format!(
                    "{} = {}.{}",
                    qualified_name, module_name, function.name
                ));
            }
        }

        let exports_str = exports.join(", ");
        let imports_str = imports.join("\n");
        let wrappers_str = wrappers.join("\n");

        format!(
            "module UserFunctions (\n    {}\n) where\n\n{}\n\n{}\n",
            exports_str, imports_str, wrappers_str
        )
    } else {
        "module UserFunctions where\n".to_string()
    };

    let user_functions_path = target_haskell_dir.join("UserFunctions.hs");
    fs::write(&user_functions_path, user_functions_content)
        .expect("Failed to write UserFunctions.hs");

    codegen_functions::generate_functions_with_modules(&file_modules);

    let haskell_build_dir = Path::new("build_scripts/haskell");
    let c_dir = Path::new("build_scripts/c");

    dispatch::generate_haskell_dispatch(&file_modules, haskell_build_dir);

    let lib_path = Path::new(&config.lib_dir);
    fs::create_dir_all(lib_path).expect("Failed to create Haskell library output directory");

    validator::validate_library_dir(lib_path);

    compiler::compile_haskell_library(
        haskell_build_dir,
        c_dir,
        &user_functions_path,
        lib_path,
        &config.lib_file,
    );

    validator::validate_main_library(&config);
    validator::warn_if_rts_missing(&config);

    compiler::copy_rts_library(&config.rts_dir, &config.rts_lib, lib_path);

    linker::emit_link_instructions(&config);
    linker::emit_rerun_instructions(&config.functions_dir);

    println!(
        "cargo:rerun-if-changed={}",
        Path::new(haskell_build_dir).join("Runtime.hs").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        Path::new(haskell_build_dir).join("Codec.hs").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        Path::new(haskell_build_dir).join("Dispatch.hs").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        Path::new(&config.functions_dir).display()
    );

    for (module_name, functions) in &file_modules {
        for function in functions {
            println!(
                "cargo:warning=Registered Haskell function: {} from module {}",
                function.name, module_name
            );
        }
    }
}
