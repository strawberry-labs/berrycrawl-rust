pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MapResponse {
    #[serde(rename = "creditsUsed")]
    #[serde(default)]
    pub credits_used: i64,
    #[serde(default)]
    pub links: Vec<MapLink>,
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub total: i64,
}

impl MapResponse {
    pub fn builder() -> MapResponseBuilder {
        <MapResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MapResponseBuilder {
    credits_used: Option<i64>,
    links: Option<Vec<MapLink>>,
    success: Option<bool>,
    total: Option<i64>,
}

impl MapResponseBuilder {
    pub fn credits_used(mut self, value: i64) -> Self {
        self.credits_used = Some(value);
        self
    }

    pub fn links(mut self, value: Vec<MapLink>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MapResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`credits_used`](MapResponseBuilder::credits_used)
    /// - [`links`](MapResponseBuilder::links)
    /// - [`success`](MapResponseBuilder::success)
    /// - [`total`](MapResponseBuilder::total)
    pub fn build(self) -> Result<MapResponse, BuildError> {
        Ok(MapResponse {
            credits_used: self
                .credits_used
                .ok_or_else(|| BuildError::missing_field("credits_used"))?,
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
