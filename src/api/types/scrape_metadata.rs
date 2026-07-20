pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScrapeMetadata {
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub url: String,
}

impl ScrapeMetadata {
    pub fn builder() -> ScrapeMetadataBuilder {
        <ScrapeMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScrapeMetadataBuilder {
    content_type: Option<String>,
    status_code: Option<i64>,
    timestamp: Option<DateTime<FixedOffset>>,
    title: Option<String>,
    url: Option<String>,
}

impl ScrapeMetadataBuilder {
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn status_code(mut self, value: i64) -> Self {
        self.status_code = Some(value);
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
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

    /// Consumes the builder and constructs a [`ScrapeMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`timestamp`](ScrapeMetadataBuilder::timestamp)
    /// - [`url`](ScrapeMetadataBuilder::url)
    pub fn build(self) -> Result<ScrapeMetadata, BuildError> {
        Ok(ScrapeMetadata {
            content_type: self.content_type,
            status_code: self.status_code,
            timestamp: self
                .timestamp
                .ok_or_else(|| BuildError::missing_field("timestamp"))?,
            title: self.title,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
