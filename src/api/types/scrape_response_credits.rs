pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScrapeResponseCredits {
    #[serde(default)]
    pub used: i64,
}

impl ScrapeResponseCredits {
    pub fn builder() -> ScrapeResponseCreditsBuilder {
        <ScrapeResponseCreditsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScrapeResponseCreditsBuilder {
    used: Option<i64>,
}

impl ScrapeResponseCreditsBuilder {
    pub fn used(mut self, value: i64) -> Self {
        self.used = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScrapeResponseCredits`].
    /// This method will fail if any of the following fields are not set:
    /// - [`used`](ScrapeResponseCreditsBuilder::used)
    pub fn build(self) -> Result<ScrapeResponseCredits, BuildError> {
        Ok(ScrapeResponseCredits {
            used: self.used.ok_or_else(|| BuildError::missing_field("used"))?,
        })
    }
}
