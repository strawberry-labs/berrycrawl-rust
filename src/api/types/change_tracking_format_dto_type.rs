pub use crate::prelude::*;

/// Format type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ChangeTrackingFormatDtoType {
    #[serde(rename = "changeTracking")]
    ChangeTracking,
}
impl fmt::Display for ChangeTrackingFormatDtoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ChangeTracking => "changeTracking",
        };
        write!(f, "{}", s)
    }
}
