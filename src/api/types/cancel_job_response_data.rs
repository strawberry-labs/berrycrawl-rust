pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelJobResponseData {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub status: String,
}

impl CancelJobResponseData {
    pub fn builder() -> CancelJobResponseDataBuilder {
        <CancelJobResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelJobResponseDataBuilder {
    id: Option<String>,
    status: Option<String>,
}

impl CancelJobResponseDataBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CancelJobResponseData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CancelJobResponseDataBuilder::id)
    /// - [`status`](CancelJobResponseDataBuilder::status)
    pub fn build(self) -> Result<CancelJobResponseData, BuildError> {
        Ok(CancelJobResponseData {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
