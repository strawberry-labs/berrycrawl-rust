pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountResponseDataPlan {
    #[serde(default)]
    pub concurrency: i64,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "queueLimit")]
    #[serde(default)]
    pub queue_limit: i64,
    #[serde(rename = "rateLimitPerMinute")]
    #[serde(default)]
    pub rate_limit_per_minute: i64,
}

impl AccountResponseDataPlan {
    pub fn builder() -> AccountResponseDataPlanBuilder {
        <AccountResponseDataPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountResponseDataPlanBuilder {
    concurrency: Option<i64>,
    id: Option<String>,
    name: Option<String>,
    queue_limit: Option<i64>,
    rate_limit_per_minute: Option<i64>,
}

impl AccountResponseDataPlanBuilder {
    pub fn concurrency(mut self, value: i64) -> Self {
        self.concurrency = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn queue_limit(mut self, value: i64) -> Self {
        self.queue_limit = Some(value);
        self
    }

    pub fn rate_limit_per_minute(mut self, value: i64) -> Self {
        self.rate_limit_per_minute = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountResponseDataPlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`concurrency`](AccountResponseDataPlanBuilder::concurrency)
    /// - [`id`](AccountResponseDataPlanBuilder::id)
    /// - [`name`](AccountResponseDataPlanBuilder::name)
    /// - [`queue_limit`](AccountResponseDataPlanBuilder::queue_limit)
    /// - [`rate_limit_per_minute`](AccountResponseDataPlanBuilder::rate_limit_per_minute)
    pub fn build(self) -> Result<AccountResponseDataPlan, BuildError> {
        Ok(AccountResponseDataPlan {
            concurrency: self
                .concurrency
                .ok_or_else(|| BuildError::missing_field("concurrency"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            queue_limit: self
                .queue_limit
                .ok_or_else(|| BuildError::missing_field("queue_limit"))?,
            rate_limit_per_minute: self
                .rate_limit_per_minute
                .ok_or_else(|| BuildError::missing_field("rate_limit_per_minute"))?,
        })
    }
}
