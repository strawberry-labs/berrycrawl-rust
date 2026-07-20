pub use crate::prelude::*;

/// Sitemap handling strategy
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CrawlDtoSitemap {
    Include,
    Skip,
    Only,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CrawlDtoSitemap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Include => serializer.serialize_str("include"),
            Self::Skip => serializer.serialize_str("skip"),
            Self::Only => serializer.serialize_str("only"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CrawlDtoSitemap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "include" => Ok(Self::Include),
            "skip" => Ok(Self::Skip),
            "only" => Ok(Self::Only),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CrawlDtoSitemap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Include => write!(f, "include"),
            Self::Skip => write!(f, "skip"),
            Self::Only => write!(f, "only"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
