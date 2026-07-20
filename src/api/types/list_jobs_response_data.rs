pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListJobsResponseData {
    #[serde(default)]
    pub jobs: Vec<JobSummary>,
    #[serde(default)]
    pub pagination: Pagination,
}

impl ListJobsResponseData {
    pub fn builder() -> ListJobsResponseDataBuilder {
        <ListJobsResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListJobsResponseDataBuilder {
    jobs: Option<Vec<JobSummary>>,
    pagination: Option<Pagination>,
}

impl ListJobsResponseDataBuilder {
    pub fn jobs(mut self, value: Vec<JobSummary>) -> Self {
        self.jobs = Some(value);
        self
    }

    pub fn pagination(mut self, value: Pagination) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListJobsResponseData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`jobs`](ListJobsResponseDataBuilder::jobs)
    /// - [`pagination`](ListJobsResponseDataBuilder::pagination)
    pub fn build(self) -> Result<ListJobsResponseData, BuildError> {
        Ok(ListJobsResponseData {
            jobs: self.jobs.ok_or_else(|| BuildError::missing_field("jobs"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
