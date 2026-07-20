pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountResponse {
    #[serde(default)]
    pub data: AccountResponseData,
    #[serde(default)]
    pub success: bool,
}

impl AccountResponse {
    pub fn builder() -> AccountResponseBuilder {
        <AccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountResponseBuilder {
    data: Option<AccountResponseData>,
    success: Option<bool>,
}

impl AccountResponseBuilder {
    pub fn data(mut self, value: AccountResponseData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](AccountResponseBuilder::data)
    /// - [`success`](AccountResponseBuilder::success)
    pub fn build(self) -> Result<AccountResponse, BuildError> {
        Ok(AccountResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
