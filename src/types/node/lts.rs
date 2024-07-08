use serde::{de::Visitor, Deserialize, Deserializer};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeLTS {
    CodeName(String),
    Bool,
}

impl NodeLTS {
    pub const fn is_code_name(&self) -> bool {
        matches!(self, Self::CodeName(_))
    }
}

impl std::fmt::Display for NodeLTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CodeName(code_name) => write!(f, "{code_name}"),
            Self::Bool => write!(f, "false"),
        }
    }
}

impl<'de> Deserialize<'de> for NodeLTS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(NodeLTSVisitor)
    }
}

struct NodeLTSVisitor;
impl<'de> Visitor<'de> for NodeLTSVisitor {
    type Value = NodeLTS;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string or a boolean")
    }

    fn visit_bool<E>(self, _value: bool) -> Result<NodeLTS, E> {
        Ok(NodeLTS::Bool)
    }

    fn visit_str<E>(self, value: &str) -> Result<NodeLTS, E> {
        Ok(NodeLTS::CodeName(value.to_string()))
    }
}
