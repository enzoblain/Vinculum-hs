use super::errors::ParseError;
use std::convert::TryFrom;

#[derive(Debug)]
pub(crate) enum HaskellType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    Char,
    Generic {
        name: String,
        rust_generic_name: String,
    },
}

impl TryFrom<&str> for HaskellType {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.chars().next().is_some_and(|c| c.is_lowercase()) {
            return Ok(Self::Generic {
                name: s.to_string(),
                rust_generic_name: String::new(),
            });
        }

        match s {
            "Int8" => Ok(Self::I8),
            "Int16" => Ok(Self::I16),
            "Int32" => Ok(Self::I32),
            "Int64" => Ok(Self::I64),
            "Word8" => Ok(Self::U8),
            "Word16" => Ok(Self::U16),
            "Word32" => Ok(Self::U32),
            "Word64" => Ok(Self::U64),
            "Float" => Ok(Self::F32),
            "Double" => Ok(Self::F64),
            "Bool" => Ok(Self::Bool),
            "Char" => Ok(Self::Char),
            _ => Err(ParseError::UnsupportedHaskellType(s.to_owned())),
        }
    }
}

impl HaskellType {
    pub(crate) fn haskell_name(&self) -> String {
        match self {
            HaskellType::I8 => "Int8".to_string(),
            HaskellType::I16 => "Int16".to_string(),
            HaskellType::I32 => "Int32".to_string(),
            HaskellType::I64 => "Int64".to_string(),
            HaskellType::U8 => "Word8".to_string(),
            HaskellType::U16 => "Word16".to_string(),
            HaskellType::U32 => "Word32".to_string(),
            HaskellType::U64 => "Word64".to_string(),
            HaskellType::F32 => "Float".to_string(),
            HaskellType::F64 => "Double".to_string(),
            HaskellType::Bool => "Bool".to_string(),
            HaskellType::Char => "Char".to_string(),
            HaskellType::Generic { .. } => "Generic".to_string(),
        }
    }

    pub(crate) fn rust_name(&self) -> String {
        match self {
            HaskellType::I8 => "i8".to_string(),
            HaskellType::I16 => "i16".to_string(),
            HaskellType::I32 => "i32".to_string(),
            HaskellType::I64 => "i64".to_string(),
            HaskellType::U8 => "u8".to_string(),
            HaskellType::U16 => "u16".to_string(),
            HaskellType::U32 => "u32".to_string(),
            HaskellType::U64 => "u64".to_string(),
            HaskellType::F32 => "f32".to_string(),
            HaskellType::F64 => "f64".to_string(),
            HaskellType::Bool => "bool".to_string(),
            HaskellType::Char => "char".to_string(),
            HaskellType::Generic {
                rust_generic_name, ..
            } => rust_generic_name.to_string(),
        }
    }

    pub(crate) fn to_rust_value(&self, name: &str) -> String {
        let type_param = if self.is_generic() { "T" } else { "()" };
        format!("Value::<{}>::{}({})", type_param, self.haskell_name(), name)
    }

    pub(crate) fn to_haskell_value(&self, name: &str) -> String {
        match self {
            HaskellType::I8
            | HaskellType::I16
            | HaskellType::I32
            | HaskellType::I64
            | HaskellType::U8
            | HaskellType::U16
            | HaskellType::U32
            | HaskellType::U64 => format!("(fromIntegral {})", name),
            _ => name.to_string(),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn from_haskell_value(&self, name: &str) -> String {
        match self {
            HaskellType::I8
            | HaskellType::I16
            | HaskellType::I32
            | HaskellType::I64
            | HaskellType::U8
            | HaskellType::U16
            | HaskellType::U32
            | HaskellType::U64 => format!("(fromIntegral ({}))", name),
            _ => name.to_string(),
        }
    }

    pub(crate) fn is_generic(&self) -> bool {
        matches!(self, HaskellType::Generic { .. })
    }
}
