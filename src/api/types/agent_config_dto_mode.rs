pub use crate::prelude::*;

/// Model routing mode. "default" uses the fast default model; "smart" uses the higher-latency reasoning model.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentConfigDtoMode {
    Default,
    Smart,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentConfigDtoMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Default => serializer.serialize_str("default"),
            Self::Smart => serializer.serialize_str("smart"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentConfigDtoMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "default" => Ok(Self::Default),
            "smart" => Ok(Self::Smart),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentConfigDtoMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::Smart => write!(f, "smart"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
