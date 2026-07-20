pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListJobsResponse {
    #[serde(default)]
    pub data: ListJobsResponseData,
    #[serde(default)]
    pub success: bool,
}

impl ListJobsResponse {
    pub fn builder() -> ListJobsResponseBuilder {
        <ListJobsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListJobsResponseBuilder {
    data: Option<ListJobsResponseData>,
    success: Option<bool>,
}

impl ListJobsResponseBuilder {
    pub fn data(mut self, value: ListJobsResponseData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListJobsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListJobsResponseBuilder::data)
    /// - [`success`](ListJobsResponseBuilder::success)
    pub fn build(self) -> Result<ListJobsResponse, BuildError> {
        Ok(ListJobsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
