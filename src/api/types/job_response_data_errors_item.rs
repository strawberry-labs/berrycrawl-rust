pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JobResponseDataErrorsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl JobResponseDataErrorsItem {
    pub fn builder() -> JobResponseDataErrorsItemBuilder {
        <JobResponseDataErrorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JobResponseDataErrorsItemBuilder {
    error: Option<String>,
    url: Option<String>,
}

impl JobResponseDataErrorsItemBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`JobResponseDataErrorsItem`].
    pub fn build(self) -> Result<JobResponseDataErrorsItem, BuildError> {
        Ok(JobResponseDataErrorsItem {
            error: self.error,
            url: self.url,
        })
    }
}
