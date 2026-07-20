pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountResponseDataQueue {
    #[serde(default)]
    pub active: i64,
    #[serde(default)]
    pub waiting: i64,
}

impl AccountResponseDataQueue {
    pub fn builder() -> AccountResponseDataQueueBuilder {
        <AccountResponseDataQueueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountResponseDataQueueBuilder {
    active: Option<i64>,
    waiting: Option<i64>,
}

impl AccountResponseDataQueueBuilder {
    pub fn active(mut self, value: i64) -> Self {
        self.active = Some(value);
        self
    }

    pub fn waiting(mut self, value: i64) -> Self {
        self.waiting = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountResponseDataQueue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`active`](AccountResponseDataQueueBuilder::active)
    /// - [`waiting`](AccountResponseDataQueueBuilder::waiting)
    pub fn build(self) -> Result<AccountResponseDataQueue, BuildError> {
        Ok(AccountResponseDataQueue {
            active: self
                .active
                .ok_or_else(|| BuildError::missing_field("active"))?,
            waiting: self
                .waiting
                .ok_or_else(|| BuildError::missing_field("waiting"))?,
        })
    }
}
