use std::fmt;

use serde::{Deserialize, Deserializer, de::Visitor};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Lts {
    CodeName(String),
    False,
}

impl Lts {
    pub const fn is_code_name(&self) -> bool {
        matches!(self, Self::CodeName(_))
    }
}

impl fmt::Display for Lts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CodeName(code_name) => write!(f, "{code_name}"),
            Self::False => write!(f, "false"),
        }
    }
}

impl<'de> Deserialize<'de> for Lts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(LtsVisitor)
    }
}

struct LtsVisitor;
impl Visitor<'_> for LtsVisitor {
    type Value = Lts;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or a boolean")
    }

    fn visit_bool<E>(self, _value: bool) -> Result<Lts, E> {
        Ok(Lts::False)
    }

    fn visit_str<E>(self, value: &str) -> Result<Lts, E> {
        Ok(Lts::CodeName(value.to_string()))
    }
}
