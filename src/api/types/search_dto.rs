pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchDto {
    /// Category filters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// Country code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Maximum number of results
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub limit: Option<f64>,
    /// Location for geo-targeting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Search query
    #[serde(default)]
    pub query: String,
    /// Source types to search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
    /// Time-based filter (e.g., qdr:d for past day)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbs: Option<String>,
    /// Timeout for search operation in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub timeout: Option<f64>,
}

impl SearchDto {
    pub fn builder() -> SearchDtoBuilder {
        <SearchDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchDtoBuilder {
    categories: Option<Vec<String>>,
    country: Option<String>,
    limit: Option<f64>,
    location: Option<String>,
    query: Option<String>,
    sources: Option<Vec<String>>,
    tbs: Option<String>,
    timeout: Option<f64>,
}

impl SearchDtoBuilder {
    pub fn categories(mut self, value: Vec<String>) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.location = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn sources(mut self, value: Vec<String>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn tbs(mut self, value: impl Into<String>) -> Self {
        self.tbs = Some(value.into());
        self
    }

    pub fn timeout(mut self, value: f64) -> Self {
        self.timeout = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchDto`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](SearchDtoBuilder::query)
    pub fn build(self) -> Result<SearchDto, BuildError> {
        Ok(SearchDto {
            categories: self.categories,
            country: self.country,
            limit: self.limit,
            location: self.location,
            query: self
                .query
                .ok_or_else(|| BuildError::missing_field("query"))?,
            sources: self.sources,
            tbs: self.tbs,
            timeout: self.timeout,
        })
    }
}
