pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JobSummaryType {
    Crawl,
    Extract,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for JobSummaryType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Crawl => serializer.serialize_str("crawl"),
            Self::Extract => serializer.serialize_str("extract"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for JobSummaryType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "crawl" => Ok(Self::Crawl),
            "extract" => Ok(Self::Extract),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for JobSummaryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Crawl => write!(f, "crawl"),
            Self::Extract => write!(f, "extract"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
