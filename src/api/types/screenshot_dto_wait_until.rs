pub use crate::prelude::*;

/// Page readiness milestone used before capture
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScreenshotDtoWaitUntil {
    Domcontentloaded,
    Load,
    Networkidle,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScreenshotDtoWaitUntil {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Domcontentloaded => serializer.serialize_str("domcontentloaded"),
            Self::Load => serializer.serialize_str("load"),
            Self::Networkidle => serializer.serialize_str("networkidle"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScreenshotDtoWaitUntil {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "domcontentloaded" => Ok(Self::Domcontentloaded),
            "load" => Ok(Self::Load),
            "networkidle" => Ok(Self::Networkidle),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScreenshotDtoWaitUntil {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Domcontentloaded => write!(f, "domcontentloaded"),
            Self::Load => write!(f, "load"),
            Self::Networkidle => write!(f, "networkidle"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
