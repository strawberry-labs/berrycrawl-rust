pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScrapeDtoFormatsItemZero {
    Markdown,
    Html,
    Rawhtml,
    Links,
    Images,
    Summary,
    Json,
    ChangeTracking,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScrapeDtoFormatsItemZero {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Markdown => serializer.serialize_str("markdown"),
            Self::Html => serializer.serialize_str("html"),
            Self::Rawhtml => serializer.serialize_str("rawhtml"),
            Self::Links => serializer.serialize_str("links"),
            Self::Images => serializer.serialize_str("images"),
            Self::Summary => serializer.serialize_str("summary"),
            Self::Json => serializer.serialize_str("json"),
            Self::ChangeTracking => serializer.serialize_str("changeTracking"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScrapeDtoFormatsItemZero {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "markdown" => Ok(Self::Markdown),
            "html" => Ok(Self::Html),
            "rawhtml" => Ok(Self::Rawhtml),
            "links" => Ok(Self::Links),
            "images" => Ok(Self::Images),
            "summary" => Ok(Self::Summary),
            "json" => Ok(Self::Json),
            "changeTracking" => Ok(Self::ChangeTracking),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScrapeDtoFormatsItemZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Markdown => write!(f, "markdown"),
            Self::Html => write!(f, "html"),
            Self::Rawhtml => write!(f, "rawhtml"),
            Self::Links => write!(f, "links"),
            Self::Images => write!(f, "images"),
            Self::Summary => write!(f, "summary"),
            Self::Json => write!(f, "json"),
            Self::ChangeTracking => write!(f, "changeTracking"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
