pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebhookResponse {
    #[serde(default)]
    pub data: Webhook,
    #[serde(default)]
    pub success: bool,
}

impl WebhookResponse {
    pub fn builder() -> WebhookResponseBuilder {
        <WebhookResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookResponseBuilder {
    data: Option<Webhook>,
    success: Option<bool>,
}

impl WebhookResponseBuilder {
    pub fn data(mut self, value: Webhook) -> Self {
        self.data = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhookResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookResponseBuilder::data)
    /// - [`success`](WebhookResponseBuilder::success)
    pub fn build(self) -> Result<WebhookResponse, BuildError> {
        Ok(WebhookResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
