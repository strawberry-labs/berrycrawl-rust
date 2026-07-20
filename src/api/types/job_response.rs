pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobResponse {
    pub data: JobResponseData,
    #[serde(default)]
    pub success: bool,
}

impl JobResponse {
    pub fn builder() -> JobResponseBuilder {
        <JobResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JobResponseBuilder {
    data: Option<JobResponseData>,
    success: Option<bool>,
}

impl JobResponseBuilder {
    pub fn data(mut self, value: JobResponseData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JobResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](JobResponseBuilder::data)
    /// - [`success`](JobResponseBuilder::success)
    pub fn build(self) -> Result<JobResponse, BuildError> {
        Ok(JobResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
