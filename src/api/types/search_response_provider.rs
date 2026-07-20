pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SearchResponseProvider {
    #[serde(rename = "parallel")]
    Parallel,
}
impl fmt::Display for SearchResponseProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Parallel => "parallel",
        };
        write!(f, "{}", s)
    }
}
