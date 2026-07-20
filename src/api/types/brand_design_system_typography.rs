pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrandDesignSystemTypography {
    #[serde(rename = "fontFamilies")]
    #[serde(default)]
    pub font_families: BrandDesignSystemTypographyFontFamilies,
    #[serde(rename = "fontSizes")]
    #[serde(default)]
    pub font_sizes: BrandDesignSystemTypographyFontSizes,
    #[serde(rename = "fontStacks")]
    #[serde(default)]
    pub font_stacks: BrandDesignSystemTypographyFontStacks,
}

impl BrandDesignSystemTypography {
    pub fn builder() -> BrandDesignSystemTypographyBuilder {
        <BrandDesignSystemTypographyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandDesignSystemTypographyBuilder {
    font_families: Option<BrandDesignSystemTypographyFontFamilies>,
    font_sizes: Option<BrandDesignSystemTypographyFontSizes>,
    font_stacks: Option<BrandDesignSystemTypographyFontStacks>,
}

impl BrandDesignSystemTypographyBuilder {
    pub fn font_families(mut self, value: BrandDesignSystemTypographyFontFamilies) -> Self {
        self.font_families = Some(value);
        self
    }

    pub fn font_sizes(mut self, value: BrandDesignSystemTypographyFontSizes) -> Self {
        self.font_sizes = Some(value);
        self
    }

    pub fn font_stacks(mut self, value: BrandDesignSystemTypographyFontStacks) -> Self {
        self.font_stacks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BrandDesignSystemTypography`].
    /// This method will fail if any of the following fields are not set:
    /// - [`font_families`](BrandDesignSystemTypographyBuilder::font_families)
    /// - [`font_sizes`](BrandDesignSystemTypographyBuilder::font_sizes)
    /// - [`font_stacks`](BrandDesignSystemTypographyBuilder::font_stacks)
    pub fn build(self) -> Result<BrandDesignSystemTypography, BuildError> {
        Ok(BrandDesignSystemTypography {
            font_families: self
                .font_families
                .ok_or_else(|| BuildError::missing_field("font_families"))?,
            font_sizes: self
                .font_sizes
                .ok_or_else(|| BuildError::missing_field("font_sizes"))?,
            font_stacks: self
                .font_stacks
                .ok_or_else(|| BuildError::missing_field("font_stacks"))?,
        })
    }
}
