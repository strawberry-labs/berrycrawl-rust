pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListWebhooksResponse {
    #[serde(default)]
    pub data: Vec<Webhook>,
    #[serde(default)]
    pub success: bool,
}

impl ListWebhooksResponse {
    pub fn builder() -> ListWebhooksResponseBuilder {
        <ListWebhooksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWebhooksResponseBuilder {
    data: Option<Vec<Webhook>>,
    success: Option<bool>,
}

impl ListWebhooksResponseBuilder {
    pub fn data(mut self, value: Vec<Webhook>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWebhooksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListWebhooksResponseBuilder::data)
    /// - [`success`](ListWebhooksResponseBuilder::success)
    pub fn build(self) -> Result<ListWebhooksResponse, BuildError> {
        Ok(ListWebhooksResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
