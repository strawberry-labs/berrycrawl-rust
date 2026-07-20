pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<SearchResultProvider>,
    #[serde(rename = "publishedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_date: Option<String>,
    #[serde(default)]
    pub snippet: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub url: String,
}

impl SearchResult {
    pub fn builder() -> SearchResultBuilder {
        <SearchResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchResultBuilder {
    provider: Option<SearchResultProvider>,
    published_date: Option<String>,
    snippet: Option<String>,
    source: Option<String>,
    title: Option<String>,
    url: Option<String>,
}

impl SearchResultBuilder {
    pub fn provider(mut self, value: SearchResultProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn published_date(mut self, value: impl Into<String>) -> Self {
        self.published_date = Some(value.into());
        self
    }

    pub fn snippet(mut self, value: impl Into<String>) -> Self {
        self.snippet = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SearchResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`snippet`](SearchResultBuilder::snippet)
    /// - [`source`](SearchResultBuilder::source)
    /// - [`title`](SearchResultBuilder::title)
    /// - [`url`](SearchResultBuilder::url)
    pub fn build(self) -> Result<SearchResult, BuildError> {
        Ok(SearchResult {
            provider: self.provider,
            published_date: self.published_date,
            snippet: self
                .snippet
                .ok_or_else(|| BuildError::missing_field("snippet"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
