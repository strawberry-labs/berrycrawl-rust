pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TestWebhookDto {
    /// Event type to test
    #[serde(default)]
    pub event: String,
    /// Optional custom payload for testing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<HashMap<String, serde_json::Value>>,
}

impl TestWebhookDto {
    pub fn builder() -> TestWebhookDtoBuilder {
        <TestWebhookDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TestWebhookDtoBuilder {
    event: Option<String>,
    payload: Option<HashMap<String, serde_json::Value>>,
}

impl TestWebhookDtoBuilder {
    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
        self
    }

    pub fn payload(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.payload = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TestWebhookDto`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event`](TestWebhookDtoBuilder::event)
    pub fn build(self) -> Result<TestWebhookDto, BuildError> {
        Ok(TestWebhookDto {
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
            payload: self.payload,
        })
    }
}
