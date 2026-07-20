pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BrandResponseData {
    #[serde(default)]
    pub brand: BrandProfile,
    #[serde(default)]
    pub meta: BrandResponseDataMeta,
}

impl BrandResponseData {
    pub fn builder() -> BrandResponseDataBuilder {
        <BrandResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandResponseDataBuilder {
    brand: Option<BrandProfile>,
    meta: Option<BrandResponseDataMeta>,
}

impl BrandResponseDataBuilder {
    pub fn brand(mut self, value: BrandProfile) -> Self {
        self.brand = Some(value);
        self
    }

    pub fn meta(mut self, value: BrandResponseDataMeta) -> Self {
        self.meta = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BrandResponseData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`brand`](BrandResponseDataBuilder::brand)
    /// - [`meta`](BrandResponseDataBuilder::meta)
    pub fn build(self) -> Result<BrandResponseData, BuildError> {
        Ok(BrandResponseData {
            brand: self
                .brand
                .ok_or_else(|| BuildError::missing_field("brand"))?,
            meta: self.meta.ok_or_else(|| BuildError::missing_field("meta"))?,
        })
    }
}
