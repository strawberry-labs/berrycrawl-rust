pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BrandDto {
    /// Ignore a cached profile and fetch the website again
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<bool>,
    /// Maximum time to spend building the profile, in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub timeout: Option<f64>,
    /// The public website whose brand profile should be extracted
    #[serde(default)]
    pub url: String,
}

impl BrandDto {
    pub fn builder() -> BrandDtoBuilder {
        <BrandDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandDtoBuilder {
    refresh: Option<bool>,
    timeout: Option<f64>,
    url: Option<String>,
}

impl BrandDtoBuilder {
    pub fn refresh(mut self, value: bool) -> Self {
        self.refresh = Some(value);
        self
    }

    pub fn timeout(mut self, value: f64) -> Self {
        self.timeout = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrandDto`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](BrandDtoBuilder::url)
    pub fn build(self) -> Result<BrandDto, BuildError> {
        Ok(BrandDto {
            refresh: self.refresh,
            timeout: self.timeout,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
