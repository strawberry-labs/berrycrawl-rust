pub use crate::prelude::*;

/// Format type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum JsonFormatDtoType {
    #[serde(rename = "json")]
    Json,
}
impl fmt::Display for JsonFormatDtoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Json => "json",
        };
        write!(f, "{}", s)
    }
}
