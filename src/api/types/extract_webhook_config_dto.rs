pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExtractWebhookConfigDto {
    /// Events to subscribe to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    /// Custom metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Optional secret used to sign direct webhook deliveries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Webhook URL
    #[serde(default)]
    pub url: String,
}

impl ExtractWebhookConfigDto {
    pub fn builder() -> ExtractWebhookConfigDtoBuilder {
        <ExtractWebhookConfigDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExtractWebhookConfigDtoBuilder {
    events: Option<Vec<String>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    secret: Option<String>,
    url: Option<String>,
}

impl ExtractWebhookConfigDtoBuilder {
    pub fn events(mut self, value: Vec<String>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExtractWebhookConfigDto`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](ExtractWebhookConfigDtoBuilder::url)
    pub fn build(self) -> Result<ExtractWebhookConfigDto, BuildError> {
        Ok(ExtractWebhookConfigDto {
            events: self.events,
            metadata: self.metadata,
            secret: self.secret,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
