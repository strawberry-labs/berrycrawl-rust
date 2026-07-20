pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BrandDesignSystem {
    #[serde(default)]
    pub colors: BrandDesignSystemColors,
    #[serde(rename = "colorScheme")]
    pub color_scheme: BrandDesignSystemColorScheme,
    #[serde(default)]
    pub components: BrandDesignSystemComponents,
    #[serde(default)]
    pub images: BrandDesignSystemImages,
    #[serde(default)]
    pub spacing: BrandDesignSystemSpacing,
    #[serde(default)]
    pub typography: BrandDesignSystemTypography,
}

impl BrandDesignSystem {
    pub fn builder() -> BrandDesignSystemBuilder {
        <BrandDesignSystemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandDesignSystemBuilder {
    colors: Option<BrandDesignSystemColors>,
    color_scheme: Option<BrandDesignSystemColorScheme>,
    components: Option<BrandDesignSystemComponents>,
    images: Option<BrandDesignSystemImages>,
    spacing: Option<BrandDesignSystemSpacing>,
    typography: Option<BrandDesignSystemTypography>,
}

impl BrandDesignSystemBuilder {
    pub fn colors(mut self, value: BrandDesignSystemColors) -> Self {
        self.colors = Some(value);
        self
    }

    pub fn color_scheme(mut self, value: BrandDesignSystemColorScheme) -> Self {
        self.color_scheme = Some(value);
        self
    }

    pub fn components(mut self, value: BrandDesignSystemComponents) -> Self {
        self.components = Some(value);
        self
    }

    pub fn images(mut self, value: BrandDesignSystemImages) -> Self {
        self.images = Some(value);
        self
    }

    pub fn spacing(mut self, value: BrandDesignSystemSpacing) -> Self {
        self.spacing = Some(value);
        self
    }

    pub fn typography(mut self, value: BrandDesignSystemTypography) -> Self {
        self.typography = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BrandDesignSystem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`colors`](BrandDesignSystemBuilder::colors)
    /// - [`color_scheme`](BrandDesignSystemBuilder::color_scheme)
    /// - [`components`](BrandDesignSystemBuilder::components)
    /// - [`images`](BrandDesignSystemBuilder::images)
    /// - [`spacing`](BrandDesignSystemBuilder::spacing)
    /// - [`typography`](BrandDesignSystemBuilder::typography)
    pub fn build(self) -> Result<BrandDesignSystem, BuildError> {
        Ok(BrandDesignSystem {
            colors: self
                .colors
                .ok_or_else(|| BuildError::missing_field("colors"))?,
            color_scheme: self
                .color_scheme
                .ok_or_else(|| BuildError::missing_field("color_scheme"))?,
            components: self
                .components
                .ok_or_else(|| BuildError::missing_field("components"))?,
            images: self
                .images
                .ok_or_else(|| BuildError::missing_field("images"))?,
            spacing: self
                .spacing
                .ok_or_else(|| BuildError::missing_field("spacing"))?,
            typography: self
                .typography
                .ok_or_else(|| BuildError::missing_field("typography"))?,
        })
    }
}
