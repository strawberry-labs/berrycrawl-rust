pub use crate::prelude::*;

/// Capture at CSS pixel size or the emulated device pixel ratio
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScreenshotDtoScale {
    Css,
    Device,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScreenshotDtoScale {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Css => serializer.serialize_str("css"),
            Self::Device => serializer.serialize_str("device"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScreenshotDtoScale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "css" => Ok(Self::Css),
            "device" => Ok(Self::Device),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScreenshotDtoScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Css => write!(f, "css"),
            Self::Device => write!(f, "device"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
