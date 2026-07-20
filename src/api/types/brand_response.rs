pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrandResponse {
    #[serde(default)]
    pub data: BrandResponseData,
    #[serde(default)]
    pub success: bool,
}

impl BrandResponse {
    pub fn builder() -> BrandResponseBuilder {
        <BrandResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandResponseBuilder {
    data: Option<BrandResponseData>,
    success: Option<bool>,
}

impl BrandResponseBuilder {
    pub fn data(mut self, value: BrandResponseData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BrandResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](BrandResponseBuilder::data)
    /// - [`success`](BrandResponseBuilder::success)
    pub fn build(self) -> Result<BrandResponse, BuildError> {
        Ok(BrandResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
