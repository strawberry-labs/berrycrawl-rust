pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct JobSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<i64>,
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "creditsUsed")]
    #[serde(default)]
    pub credits_used: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub total: i64,
    pub r#type: JobSummaryType,
}

impl JobSummary {
    pub fn builder() -> JobSummaryBuilder {
        <JobSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JobSummaryBuilder {
    completed: Option<i64>,
    completed_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    credits_used: Option<i64>,
    error: Option<String>,
    failed: Option<i64>,
    id: Option<String>,
    started_at: Option<DateTime<FixedOffset>>,
    status: Option<String>,
    total: Option<i64>,
    r#type: Option<JobSummaryType>,
}

impl JobSummaryBuilder {
    pub fn completed(mut self, value: i64) -> Self {
        self.completed = Some(value);
        self
    }

    pub fn completed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn credits_used(mut self, value: i64) -> Self {
        self.credits_used = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn failed(mut self, value: i64) -> Self {
        self.failed = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn r#type(mut self, value: JobSummaryType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JobSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](JobSummaryBuilder::created_at)
    /// - [`credits_used`](JobSummaryBuilder::credits_used)
    /// - [`id`](JobSummaryBuilder::id)
    /// - [`status`](JobSummaryBuilder::status)
    /// - [`total`](JobSummaryBuilder::total)
    /// - [`r#type`](JobSummaryBuilder::r#type)
    pub fn build(self) -> Result<JobSummary, BuildError> {
        Ok(JobSummary {
            completed: self.completed,
            completed_at: self.completed_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            credits_used: self
                .credits_used
                .ok_or_else(|| BuildError::missing_field("credits_used"))?,
            error: self.error,
            failed: self.failed,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            started_at: self.started_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
