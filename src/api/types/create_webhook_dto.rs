pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWebhookDto {
    /// Event types to subscribe to
    #[serde(default)]
    pub events: Vec<String>,
    /// Webhook URL to send events to
    #[serde(default)]
    pub url: String,
}

impl CreateWebhookDto {
    pub fn builder() -> CreateWebhookDtoBuilder {
        <CreateWebhookDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWebhookDtoBuilder {
    events: Option<Vec<String>>,
    url: Option<String>,
}

impl CreateWebhookDtoBuilder {
    pub fn events(mut self, value: Vec<String>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateWebhookDto`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](CreateWebhookDtoBuilder::events)
    /// - [`url`](CreateWebhookDtoBuilder::url)
    pub fn build(self) -> Result<CreateWebhookDto, BuildError> {
        Ok(CreateWebhookDto {
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
