pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TestWebhookResponse {
    #[serde(default)]
    pub event: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub payload: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub success: bool,
}

impl TestWebhookResponse {
    pub fn builder() -> TestWebhookResponseBuilder {
        <TestWebhookResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TestWebhookResponseBuilder {
    event: Option<String>,
    message: Option<String>,
    payload: Option<HashMap<String, serde_json::Value>>,
    success: Option<bool>,
}

impl TestWebhookResponseBuilder {
    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn payload(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.payload = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TestWebhookResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event`](TestWebhookResponseBuilder::event)
    /// - [`message`](TestWebhookResponseBuilder::message)
    /// - [`payload`](TestWebhookResponseBuilder::payload)
    /// - [`success`](TestWebhookResponseBuilder::success)
    pub fn build(self) -> Result<TestWebhookResponse, BuildError> {
        Ok(TestWebhookResponse {
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            payload: self
                .payload
                .ok_or_else(|| BuildError::missing_field("payload"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
