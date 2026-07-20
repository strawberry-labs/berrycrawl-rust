pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActionDtoType {
    Wait,
    Click,
    Write,
    Press,
    Scroll,
    Scrape,
    Screenshot,
    Pdf,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ActionDtoType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Wait => serializer.serialize_str("wait"),
            Self::Click => serializer.serialize_str("click"),
            Self::Write => serializer.serialize_str("write"),
            Self::Press => serializer.serialize_str("press"),
            Self::Scroll => serializer.serialize_str("scroll"),
            Self::Scrape => serializer.serialize_str("scrape"),
            Self::Screenshot => serializer.serialize_str("screenshot"),
            Self::Pdf => serializer.serialize_str("pdf"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ActionDtoType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "wait" => Ok(Self::Wait),
            "click" => Ok(Self::Click),
            "write" => Ok(Self::Write),
            "press" => Ok(Self::Press),
            "scroll" => Ok(Self::Scroll),
            "scrape" => Ok(Self::Scrape),
            "screenshot" => Ok(Self::Screenshot),
            "pdf" => Ok(Self::Pdf),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ActionDtoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wait => write!(f, "wait"),
            Self::Click => write!(f, "click"),
            Self::Write => write!(f, "write"),
            Self::Press => write!(f, "press"),
            Self::Scroll => write!(f, "scroll"),
            Self::Scrape => write!(f, "scrape"),
            Self::Screenshot => write!(f, "screenshot"),
            Self::Pdf => write!(f, "pdf"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
