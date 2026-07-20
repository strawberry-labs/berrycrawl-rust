pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebhookConfigDto {
    /// Events to subscribe to
    #[serde(default)]
    pub events: Vec<String>,
    /// Additional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Optional secret used to sign direct webhook deliveries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Webhook URL
    #[serde(default)]
    pub url: String,
}

impl WebhookConfigDto {
    pub fn builder() -> WebhookConfigDtoBuilder {
        <WebhookConfigDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookConfigDtoBuilder {
    events: Option<Vec<String>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    secret: Option<String>,
    url: Option<String>,
}

impl WebhookConfigDtoBuilder {
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

    /// Consumes the builder and constructs a [`WebhookConfigDto`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](WebhookConfigDtoBuilder::events)
    /// - [`url`](WebhookConfigDtoBuilder::url)
    pub fn build(self) -> Result<WebhookConfigDto, BuildError> {
        Ok(WebhookConfigDto {
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            metadata: self.metadata,
            secret: self.secret,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
