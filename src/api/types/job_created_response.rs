pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JobCreatedResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "invalidURLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_ur_ls: Option<Vec<String>>,
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub url: String,
}

impl JobCreatedResponse {
    pub fn builder() -> JobCreatedResponseBuilder {
        <JobCreatedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JobCreatedResponseBuilder {
    id: Option<String>,
    invalid_ur_ls: Option<Vec<String>>,
    success: Option<bool>,
    url: Option<String>,
}

impl JobCreatedResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn invalid_ur_ls(mut self, value: Vec<String>) -> Self {
        self.invalid_ur_ls = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`JobCreatedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](JobCreatedResponseBuilder::id)
    /// - [`success`](JobCreatedResponseBuilder::success)
    /// - [`url`](JobCreatedResponseBuilder::url)
    pub fn build(self) -> Result<JobCreatedResponse, BuildError> {
        Ok(JobCreatedResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invalid_ur_ls: self.invalid_ur_ls,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
