pub use crate::prelude::*;

/// Named viewport preset
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScreenshotDtoDevice {
    Desktop,
    DesktopHd,
    Tablet,
    Iphone15,
    Pixel8,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScreenshotDtoDevice {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Desktop => serializer.serialize_str("desktop"),
            Self::DesktopHd => serializer.serialize_str("desktop-hd"),
            Self::Tablet => serializer.serialize_str("tablet"),
            Self::Iphone15 => serializer.serialize_str("iphone-15"),
            Self::Pixel8 => serializer.serialize_str("pixel-8"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScreenshotDtoDevice {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "desktop" => Ok(Self::Desktop),
            "desktop-hd" => Ok(Self::DesktopHd),
            "tablet" => Ok(Self::Tablet),
            "iphone-15" => Ok(Self::Iphone15),
            "pixel-8" => Ok(Self::Pixel8),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScreenshotDtoDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Desktop => write!(f, "desktop"),
            Self::DesktopHd => write!(f, "desktop-hd"),
            Self::Tablet => write!(f, "tablet"),
            Self::Iphone15 => write!(f, "iphone-15"),
            Self::Pixel8 => write!(f, "pixel-8"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
