use std::fs;
use std::path::Path;

use super::types::{Arg, Function, Type};
use super::validator::validate_functions;

#[derive(Debug)]
struct HsArg {
    name: String,
    r#type: String,
}

#[derive(Debug)]
struct HsFunction {
    name: String,
    args: Vec<HsArg>,
    return_type: String,
}

pub(crate) fn parse_haskell_functions(path: impl AsRef<Path>) -> Vec<Function> {
    let content = fs::read_to_string(path.as_ref()).expect("Failed to read configuration file");
    let mut functions = Vec::new();

    let mut current: Option<String> = None;

    for raw_line in content.lines() {
        let line = strip_comment(raw_line).trim_end();

        if line.trim().is_empty() {
            continue;
        }

        if is_signature(line) {
            flush(&mut current, &mut functions);
            current = Some(line.trim().to_string());
            continue;
        }

        if let Some(buf) = &mut current {
            if raw_line.starts_with(char::is_whitespace) {
                buf.push(' ');
                buf.push_str(line.trim());
                continue;
            }

            flush(&mut current, &mut functions);
        }
    }

    flush(&mut current, &mut functions);

    // Convert HsFunction to Function with type validation
    let typed_functions: Vec<Function> = functions
        .into_iter()
        .filter_map(convert_to_typed_function)
        .collect();

    validate_functions(&typed_functions);

    typed_functions
}

fn strip_comment(line: &str) -> &str {
    line.split_once("--").map(|(code, _)| code).unwrap_or(line)
}

fn is_signature(line: &str) -> bool {
    line.contains("::")
}

fn flush(current: &mut Option<String>, functions: &mut Vec<HsFunction>) {
    if let Some(buf) = current.take()
        && let Some(function) = parse_function_signature(&buf)
    {
        functions.push(function);
    }
}

fn parse_function_signature(signature: &str) -> Option<HsFunction> {
    let (name, type_expr) = signature.split_once("::")?;
    let name = name.trim().to_string();

    let parts: Vec<&str> = type_expr
        .split("->")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if parts.is_empty() {
        return None;
    }

    if parts.len() == 1 {
        return Some(HsFunction {
            name,
            args: Vec::new(),
            return_type: parts[0].to_string(),
        });
    }

    let return_type = parts.last()?.to_string();

    let args = parts[..parts.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, ty)| HsArg {
            name: format!("arg{}", i + 1),
            r#type: ty.to_string(),
        })
        .collect();

    Some(HsFunction {
        name,
        args,
        return_type,
    })
}

fn convert_to_typed_function(hs_func: HsFunction) -> Option<Function> {
    let return_type = parse_haskell_type(&hs_func.return_type)?;
    let args = hs_func
        .args
        .into_iter()
        .map(|arg| {
            let ty = parse_haskell_type(&arg.r#type).unwrap_or(Type::String);
            Arg {
                name: arg.name,
                r#type: ty,
            }
        })
        .collect();

    Some(Function {
        name: hs_func.name,
        args,
        r#return: return_type,
    })
}

fn parse_haskell_type(type_str: &str) -> Option<Type> {
    match type_str {
        "Int8" => Some(Type::Int8),
        "Int16" => Some(Type::Int16),
        "Int32" => Some(Type::Int32),
        "Int64" => Some(Type::Int64),
        "Word8" => Some(Type::Word8),
        "Word16" => Some(Type::Word16),
        "Word32" => Some(Type::Word32),
        "Word64" => Some(Type::Word64),
        "Float32" | "Float" => Some(Type::Float32),
        "Float64" | "Double" => Some(Type::Float64),
        "Bool" => Some(Type::Bool),
        "Char" => Some(Type::Char),
        "String" => Some(Type::String),
        "ByteString" | "Bytes" => Some(Type::Bytes),
        _ => None,
    }
}
