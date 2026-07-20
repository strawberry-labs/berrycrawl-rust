pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrandAsset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

impl BrandAsset {
    pub fn builder() -> BrandAssetBuilder {
        <BrandAssetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandAssetBuilder {
    height: Option<i64>,
    theme: Option<String>,
    r#type: Option<String>,
    url: Option<String>,
    width: Option<i64>,
}

impl BrandAssetBuilder {
    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn theme(mut self, value: impl Into<String>) -> Self {
        self.theme = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BrandAsset`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](BrandAssetBuilder::r#type)
    /// - [`url`](BrandAssetBuilder::url)
    pub fn build(self) -> Result<BrandAsset, BuildError> {
        Ok(BrandAsset {
            height: self.height,
            theme: self.theme,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            width: self.width,
        })
    }
}
