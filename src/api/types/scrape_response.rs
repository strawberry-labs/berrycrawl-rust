pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScrapeResponse {
    #[serde(default)]
    pub credits: ScrapeResponseCredits,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    pub metadata: ScrapeMetadata,
    #[serde(default)]
    pub success: bool,
}

impl ScrapeResponse {
    pub fn builder() -> ScrapeResponseBuilder {
        <ScrapeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScrapeResponseBuilder {
    credits: Option<ScrapeResponseCredits>,
    data: Option<HashMap<String, serde_json::Value>>,
    error: Option<String>,
    metadata: Option<ScrapeMetadata>,
    success: Option<bool>,
}

impl ScrapeResponseBuilder {
    pub fn credits(mut self, value: ScrapeResponseCredits) -> Self {
        self.credits = Some(value);
        self
    }

    pub fn data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: ScrapeMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScrapeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`credits`](ScrapeResponseBuilder::credits)
    /// - [`metadata`](ScrapeResponseBuilder::metadata)
    /// - [`success`](ScrapeResponseBuilder::success)
    pub fn build(self) -> Result<ScrapeResponse, BuildError> {
        Ok(ScrapeResponse {
            credits: self
                .credits
                .ok_or_else(|| BuildError::missing_field("credits"))?,
            data: self.data,
            error: self.error,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
