pub use crate::prelude::*;

/// Proxy mode. auto starts direct and escalates only when blocked. basic is an alias for none.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScrapeDtoProxy {
    None,
    Basic,
    Datacenter,
    Residential,
    Stealth,
    Auto,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScrapeDtoProxy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::Basic => serializer.serialize_str("basic"),
            Self::Datacenter => serializer.serialize_str("datacenter"),
            Self::Residential => serializer.serialize_str("residential"),
            Self::Stealth => serializer.serialize_str("stealth"),
            Self::Auto => serializer.serialize_str("auto"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScrapeDtoProxy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "none" => Ok(Self::None),
            "basic" => Ok(Self::Basic),
            "datacenter" => Ok(Self::Datacenter),
            "residential" => Ok(Self::Residential),
            "stealth" => Ok(Self::Stealth),
            "auto" => Ok(Self::Auto),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScrapeDtoProxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Basic => write!(f, "basic"),
            Self::Datacenter => write!(f, "datacenter"),
            Self::Residential => write!(f, "residential"),
            Self::Stealth => write!(f, "stealth"),
            Self::Auto => write!(f, "auto"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
