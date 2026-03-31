use super::errors::ConfigError;
use super::types::Function;

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
        }
    }

    Ok(())
}
