pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MessageResponse {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub success: bool,
}

impl MessageResponse {
    pub fn builder() -> MessageResponseBuilder {
        <MessageResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageResponseBuilder {
    message: Option<String>,
    success: Option<bool>,
}

impl MessageResponseBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessageResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](MessageResponseBuilder::message)
    /// - [`success`](MessageResponseBuilder::success)
    pub fn build(self) -> Result<MessageResponse, BuildError> {
        Ok(MessageResponse {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
