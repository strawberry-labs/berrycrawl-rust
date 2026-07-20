pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrandDesignSystemImages {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(rename = "ogImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub og_image: Option<String>,
}

impl BrandDesignSystemImages {
    pub fn builder() -> BrandDesignSystemImagesBuilder {
        <BrandDesignSystemImagesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandDesignSystemImagesBuilder {
    favicon: Option<String>,
    logo: Option<String>,
    og_image: Option<String>,
}

impl BrandDesignSystemImagesBuilder {
    pub fn favicon(mut self, value: impl Into<String>) -> Self {
        self.favicon = Some(value.into());
        self
    }

    pub fn logo(mut self, value: impl Into<String>) -> Self {
        self.logo = Some(value.into());
        self
    }

    pub fn og_image(mut self, value: impl Into<String>) -> Self {
        self.og_image = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrandDesignSystemImages`].
    pub fn build(self) -> Result<BrandDesignSystemImages, BuildError> {
        Ok(BrandDesignSystemImages {
            favicon: self.favicon,
            logo: self.logo,
            og_image: self.og_image,
        })
    }
}
