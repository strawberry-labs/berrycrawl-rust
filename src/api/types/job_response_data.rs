pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobResponseData {
    #[serde(flatten)]
    pub job_summary_fields: JobSummary,
    #[serde(default)]
    pub errors: Vec<JobResponseDataErrorsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(rename = "resultPagination")]
    #[serde(default)]
    pub result_pagination: Pagination,
    #[serde(default)]
    pub results: Vec<HashMap<String, serde_json::Value>>,
}

impl JobResponseData {
    pub fn builder() -> JobResponseDataBuilder {
        <JobResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JobResponseDataBuilder {
    job_summary_fields: Option<JobSummary>,
    errors: Option<Vec<JobResponseDataErrorsItem>>,
    result: Option<serde_json::Value>,
    result_pagination: Option<Pagination>,
    results: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl JobResponseDataBuilder {
    pub fn job_summary_fields(mut self, value: JobSummary) -> Self {
        self.job_summary_fields = Some(value);
        self
    }

    pub fn errors(mut self, value: Vec<JobResponseDataErrorsItem>) -> Self {
        self.errors = Some(value);
        self
    }

    pub fn result(mut self, value: serde_json::Value) -> Self {
        self.result = Some(value);
        self
    }

    pub fn result_pagination(mut self, value: Pagination) -> Self {
        self.result_pagination = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JobResponseData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`job_summary_fields`](JobResponseDataBuilder::job_summary_fields)
    /// - [`errors`](JobResponseDataBuilder::errors)
    /// - [`result_pagination`](JobResponseDataBuilder::result_pagination)
    /// - [`results`](JobResponseDataBuilder::results)
    pub fn build(self) -> Result<JobResponseData, BuildError> {
        Ok(JobResponseData {
            job_summary_fields: self
                .job_summary_fields
                .ok_or_else(|| BuildError::missing_field("job_summary_fields"))?,
            errors: self
                .errors
                .ok_or_else(|| BuildError::missing_field("errors"))?,
            result: self.result,
            result_pagination: self
                .result_pagination
                .ok_or_else(|| BuildError::missing_field("result_pagination"))?,
            results: self
                .results
                .ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
