pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrandResponseDataMeta {
    #[serde(default)]
    pub cached: bool,
    #[serde(rename = "creditsUsed")]
    #[serde(default)]
    pub credits_used: i64,
    #[serde(rename = "fetchedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub fetched_at: DateTime<FixedOffset>,
    #[serde(rename = "sourceUrl")]
    #[serde(default)]
    pub source_url: String,
}

impl BrandResponseDataMeta {
    pub fn builder() -> BrandResponseDataMetaBuilder {
        <BrandResponseDataMetaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandResponseDataMetaBuilder {
    cached: Option<bool>,
    credits_used: Option<i64>,
    fetched_at: Option<DateTime<FixedOffset>>,
    source_url: Option<String>,
}

impl BrandResponseDataMetaBuilder {
    pub fn cached(mut self, value: bool) -> Self {
        self.cached = Some(value);
        self
    }

    pub fn credits_used(mut self, value: i64) -> Self {
        self.credits_used = Some(value);
        self
    }

    pub fn fetched_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.fetched_at = Some(value);
        self
    }

    pub fn source_url(mut self, value: impl Into<String>) -> Self {
        self.source_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrandResponseDataMeta`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cached`](BrandResponseDataMetaBuilder::cached)
    /// - [`credits_used`](BrandResponseDataMetaBuilder::credits_used)
    /// - [`fetched_at`](BrandResponseDataMetaBuilder::fetched_at)
    /// - [`source_url`](BrandResponseDataMetaBuilder::source_url)
    pub fn build(self) -> Result<BrandResponseDataMeta, BuildError> {
        Ok(BrandResponseDataMeta {
            cached: self
                .cached
                .ok_or_else(|| BuildError::missing_field("cached"))?,
            credits_used: self
                .credits_used
                .ok_or_else(|| BuildError::missing_field("credits_used"))?,
            fetched_at: self
                .fetched_at
                .ok_or_else(|| BuildError::missing_field("fetched_at"))?,
            source_url: self
                .source_url
                .ok_or_else(|| BuildError::missing_field("source_url"))?,
        })
    }
}
