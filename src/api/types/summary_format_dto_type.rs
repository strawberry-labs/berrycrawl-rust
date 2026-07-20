pub use crate::prelude::*;

/// Format type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SummaryFormatDtoType {
    #[serde(rename = "summary")]
    Summary,
}
impl fmt::Display for SummaryFormatDtoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Summary => "summary",
        };
        write!(f, "{}", s)
    }
}
