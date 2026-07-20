pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelJobResponse {
    #[serde(default)]
    pub data: CancelJobResponseData,
    #[serde(default)]
    pub success: bool,
}

impl CancelJobResponse {
    pub fn builder() -> CancelJobResponseBuilder {
        <CancelJobResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelJobResponseBuilder {
    data: Option<CancelJobResponseData>,
    success: Option<bool>,
}

impl CancelJobResponseBuilder {
    pub fn data(mut self, value: CancelJobResponseData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CancelJobResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](CancelJobResponseBuilder::data)
    /// - [`success`](CancelJobResponseBuilder::success)
    pub fn build(self) -> Result<CancelJobResponse, BuildError> {
        Ok(CancelJobResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
