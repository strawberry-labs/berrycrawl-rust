pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrandDesignSystemTypographyFontFamilies {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,
}

impl BrandDesignSystemTypographyFontFamilies {
    pub fn builder() -> BrandDesignSystemTypographyFontFamiliesBuilder {
        <BrandDesignSystemTypographyFontFamiliesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandDesignSystemTypographyFontFamiliesBuilder {
    heading: Option<String>,
    primary: Option<String>,
}

impl BrandDesignSystemTypographyFontFamiliesBuilder {
    pub fn heading(mut self, value: impl Into<String>) -> Self {
        self.heading = Some(value.into());
        self
    }

    pub fn primary(mut self, value: impl Into<String>) -> Self {
        self.primary = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrandDesignSystemTypographyFontFamilies`].
    pub fn build(self) -> Result<BrandDesignSystemTypographyFontFamilies, BuildError> {
        Ok(BrandDesignSystemTypographyFontFamilies {
            heading: self.heading,
            primary: self.primary,
        })
    }
}
