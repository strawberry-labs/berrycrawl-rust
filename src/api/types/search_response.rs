pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SearchResponse {
    #[serde(rename = "creditsUsed")]
    #[serde(default)]
    pub credits_used: i64,
    #[serde(default)]
    pub data: Vec<SearchResult>,
    pub provider: SearchResponseProvider,
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub total: i64,
}

impl SearchResponse {
    pub fn builder() -> SearchResponseBuilder {
        <SearchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchResponseBuilder {
    credits_used: Option<i64>,
    data: Option<Vec<SearchResult>>,
    provider: Option<SearchResponseProvider>,
    query: Option<String>,
    success: Option<bool>,
    total: Option<i64>,
}

impl SearchResponseBuilder {
    pub fn credits_used(mut self, value: i64) -> Self {
        self.credits_used = Some(value);
        self
    }

    pub fn data(mut self, value: Vec<SearchResult>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn provider(mut self, value: SearchResponseProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
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

    /// Consumes the builder and constructs a [`SearchResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`credits_used`](SearchResponseBuilder::credits_used)
    /// - [`data`](SearchResponseBuilder::data)
    /// - [`provider`](SearchResponseBuilder::provider)
    /// - [`query`](SearchResponseBuilder::query)
    /// - [`success`](SearchResponseBuilder::success)
    /// - [`total`](SearchResponseBuilder::total)
    pub fn build(self) -> Result<SearchResponse, BuildError> {
        Ok(SearchResponse {
            credits_used: self
                .credits_used
                .ok_or_else(|| BuildError::missing_field("credits_used"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            query: self
                .query
                .ok_or_else(|| BuildError::missing_field("query"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
