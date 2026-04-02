use std::convert::TryFrom;

use super::errors::ParseError;

#[derive(Debug)]
pub(crate) enum Type {
    Int8,
    Int16,
    Int32,
    Int64,
    Word8,
    Word16,
    Word32,
    Word64,
    Float32,
    Float64,
    Bool,
    Char,
}

impl TryFrom<&str> for Type {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Int8" => Ok(Self::Int8),
            "Int16" => Ok(Self::Int16),
            "Int32" => Ok(Self::Int32),
            "Int64" => Ok(Self::Int64),

            "Word8" => Ok(Self::Word8),
            "Word16" => Ok(Self::Word16),
            "Word32" => Ok(Self::Word32),
            "Word64" => Ok(Self::Word64),

            "Float" => Ok(Self::Float32),
            "Double" => Ok(Self::Float64),

            "Bool" => Ok(Self::Bool),
            "Char" => Ok(Self::Char),

            _ => Err(ParseError::UnsupportedType(s.to_owned())),
        }
    }
}

impl Type {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Type::Int8 => "Int8",
            Type::Int16 => "Int16",
            Type::Int32 => "Int32",
            Type::Int64 => "Int64",
            Type::Word8 => "Word8",
            Type::Word16 => "Word16",
            Type::Word32 => "Word32",
            Type::Word64 => "Word64",
            Type::Float32 => "Float",
            Type::Float64 => "Double",
            Type::Bool => "Bool",
            Type::Char => "Char",
        }
    }

    pub(crate) fn rust_type(&self) -> String {
        match self {
            Type::Int8 => "i8".to_string(),
            Type::Int16 => "i16".to_string(),
            Type::Int32 => "i32".to_string(),
            Type::Int64 => "i64".to_string(),
            Type::Word8 => "u8".to_string(),
            Type::Word16 => "u16".to_string(),
            Type::Word32 => "u32".to_string(),
            Type::Word64 => "u64".to_string(),
            Type::Float32 => "f32".to_string(),
            Type::Float64 => "f64".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Char => "char".to_string(),
        }
    }

    pub(crate) fn return_converter(&self) -> String {
        format!("into_{}", self.as_str().to_lowercase())
    }

    pub(crate) fn rust_value_ctor(&self, name: &str) -> String {
        format!("Value::{}({})", self.as_str(), name)
    }

    pub(crate) fn haskell_arg_expr(&self, name: &str) -> String {
        match self {
            Type::Int8
            | Type::Int16
            | Type::Int32
            | Type::Int64
            | Type::Word8
            | Type::Word16
            | Type::Word32
            | Type::Word64 => format!("(fromIntegral {})", name),
            _ => name.to_string(),
        }
    }

    pub(crate) fn haskell_return_expr(&self, name: &str) -> String {
        match self {
            Type::Int8
            | Type::Int16
            | Type::Int32
            | Type::Int64
            | Type::Word8
            | Type::Word16
            | Type::Word32
            | Type::Word64 => format!("(fromIntegral ({}))", name),
            _ => name.to_string(),
        }
    }
}
