pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrandDesignSystemColors {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,
    #[serde(rename = "textPrimary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_primary: Option<String>,
}

impl BrandDesignSystemColors {
    pub fn builder() -> BrandDesignSystemColorsBuilder {
        <BrandDesignSystemColorsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandDesignSystemColorsBuilder {
    accent: Option<String>,
    background: Option<String>,
    link: Option<String>,
    primary: Option<String>,
    secondary: Option<String>,
    text_primary: Option<String>,
}

impl BrandDesignSystemColorsBuilder {
    pub fn accent(mut self, value: impl Into<String>) -> Self {
        self.accent = Some(value.into());
        self
    }

    pub fn background(mut self, value: impl Into<String>) -> Self {
        self.background = Some(value.into());
        self
    }

    pub fn link(mut self, value: impl Into<String>) -> Self {
        self.link = Some(value.into());
        self
    }

    pub fn primary(mut self, value: impl Into<String>) -> Self {
        self.primary = Some(value.into());
        self
    }

    pub fn secondary(mut self, value: impl Into<String>) -> Self {
        self.secondary = Some(value.into());
        self
    }

    pub fn text_primary(mut self, value: impl Into<String>) -> Self {
        self.text_primary = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrandDesignSystemColors`].
    pub fn build(self) -> Result<BrandDesignSystemColors, BuildError> {
        Ok(BrandDesignSystemColors {
            accent: self.accent,
            background: self.background,
            link: self.link,
            primary: self.primary,
            secondary: self.secondary,
            text_primary: self.text_primary,
        })
    }
}
