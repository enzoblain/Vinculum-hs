use super::errors::ConfigError;
use super::types::{Function, Type};

pub(crate) fn validate_functions(functions: &[Function]) -> Result<(), ConfigError> {
    if functions.is_empty() {
        return Err(ConfigError::NoFunctionsDefined);
    }

    for func in functions {
        if func.name.trim().is_empty() {
            return Err(ConfigError::EmptyFunctionName);
        }

        for arg in &func.args {
            if arg.name.trim().is_empty() {
                return Err(ConfigError::EmptyArgumentName {
                    function: func.name.clone(),
                });
            }
            validate_type(&arg.r#type, &func.name)?;
        }
        validate_type(&func.r#return, &func.name)?;
    }

    Ok(())
}

fn validate_type(ty: &Type, func_name: &str) -> Result<(), ConfigError> {
    match ty {
        Type::Maybe(inner) => {
            if matches!(**inner, Type::Maybe(_)) {
                return Err(ConfigError::ValidationError(format!(
                    "Nested Maybe types are not supported in function '{}'",
                    func_name
                )));
            }
            Ok(())
        }
        _ => Ok(()),
    }
}
