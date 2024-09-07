use std::fmt;

use serde::{de::Visitor, Deserialize, Deserializer};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LTS {
    CodeName(String),
    Bool,
}

impl LTS {
    pub const fn is_code_name(&self) -> bool {
        matches!(self, Self::CodeName(_))
    }
}

impl fmt::Display for LTS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CodeName(code_name) => write!(f, "{code_name}"),
            Self::Bool => write!(f, "false"),
        }
    }
}

impl<'de> Deserialize<'de> for LTS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(LTSVisitor)
    }
}

struct LTSVisitor;
impl<'de> Visitor<'de> for LTSVisitor {
    type Value = LTS;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or a boolean")
    }

    fn visit_bool<E>(self, _value: bool) -> Result<LTS, E> {
        Ok(LTS::Bool)
    }

    fn visit_str<E>(self, value: &str) -> Result<LTS, E> {
        Ok(LTS::CodeName(value.to_string()))
    }
}
