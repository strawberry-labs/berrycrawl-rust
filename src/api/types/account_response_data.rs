pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountResponseData {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub credits: i64,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "lifetimeSpendUsd")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub lifetime_spend_usd: f64,
    #[serde(default)]
    pub plan: AccountResponseDataPlan,
    #[serde(default)]
    pub queue: AccountResponseDataQueue,
}

impl AccountResponseData {
    pub fn builder() -> AccountResponseDataBuilder {
        <AccountResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountResponseDataBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    credits: Option<i64>,
    email: Option<String>,
    id: Option<String>,
    lifetime_spend_usd: Option<f64>,
    plan: Option<AccountResponseDataPlan>,
    queue: Option<AccountResponseDataQueue>,
}

impl AccountResponseDataBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn credits(mut self, value: i64) -> Self {
        self.credits = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn lifetime_spend_usd(mut self, value: f64) -> Self {
        self.lifetime_spend_usd = Some(value);
        self
    }

    pub fn plan(mut self, value: AccountResponseDataPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn queue(mut self, value: AccountResponseDataQueue) -> Self {
        self.queue = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountResponseData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](AccountResponseDataBuilder::created_at)
    /// - [`credits`](AccountResponseDataBuilder::credits)
    /// - [`email`](AccountResponseDataBuilder::email)
    /// - [`id`](AccountResponseDataBuilder::id)
    /// - [`lifetime_spend_usd`](AccountResponseDataBuilder::lifetime_spend_usd)
    /// - [`plan`](AccountResponseDataBuilder::plan)
    /// - [`queue`](AccountResponseDataBuilder::queue)
    pub fn build(self) -> Result<AccountResponseData, BuildError> {
        Ok(AccountResponseData {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            credits: self
                .credits
                .ok_or_else(|| BuildError::missing_field("credits"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            lifetime_spend_usd: self
                .lifetime_spend_usd
                .ok_or_else(|| BuildError::missing_field("lifetime_spend_usd"))?,
            plan: self.plan.ok_or_else(|| BuildError::missing_field("plan"))?,
            queue: self
                .queue
                .ok_or_else(|| BuildError::missing_field("queue"))?,
        })
    }
}
