pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWebhookDto {
    /// Event types to subscribe to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    /// Enable or disable webhook
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Webhook URL to send events to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl UpdateWebhookDto {
    pub fn builder() -> UpdateWebhookDtoBuilder {
        <UpdateWebhookDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWebhookDtoBuilder {
    events: Option<Vec<String>>,
    is_active: Option<bool>,
    url: Option<String>,
}

impl UpdateWebhookDtoBuilder {
    pub fn events(mut self, value: Vec<String>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateWebhookDto`].
    pub fn build(self) -> Result<UpdateWebhookDto, BuildError> {
        Ok(UpdateWebhookDto {
            events: self.events,
            is_active: self.is_active,
            url: self.url,
        })
    }
}
