pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrandProfile {
    #[serde(default)]
    pub colors: Vec<BrandProfileColorsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub fonts: Vec<BrandProfileFontsItem>,
    #[serde(default)]
    pub images: Vec<BrandAsset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default)]
    pub logos: Vec<BrandAsset>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub socials: Vec<BrandProfileSocialsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagline: Option<String>,
}

impl BrandProfile {
    pub fn builder() -> BrandProfileBuilder {
        <BrandProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandProfileBuilder {
    colors: Option<Vec<BrandProfileColorsItem>>,
    description: Option<String>,
    domain: Option<String>,
    fonts: Option<Vec<BrandProfileFontsItem>>,
    images: Option<Vec<BrandAsset>>,
    language: Option<String>,
    logos: Option<Vec<BrandAsset>>,
    name: Option<String>,
    socials: Option<Vec<BrandProfileSocialsItem>>,
    tagline: Option<String>,
}

impl BrandProfileBuilder {
    pub fn colors(mut self, value: Vec<BrandProfileColorsItem>) -> Self {
        self.colors = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn fonts(mut self, value: Vec<BrandProfileFontsItem>) -> Self {
        self.fonts = Some(value);
        self
    }

    pub fn images(mut self, value: Vec<BrandAsset>) -> Self {
        self.images = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn logos(mut self, value: Vec<BrandAsset>) -> Self {
        self.logos = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn socials(mut self, value: Vec<BrandProfileSocialsItem>) -> Self {
        self.socials = Some(value);
        self
    }

    pub fn tagline(mut self, value: impl Into<String>) -> Self {
        self.tagline = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrandProfile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`colors`](BrandProfileBuilder::colors)
    /// - [`domain`](BrandProfileBuilder::domain)
    /// - [`fonts`](BrandProfileBuilder::fonts)
    /// - [`images`](BrandProfileBuilder::images)
    /// - [`logos`](BrandProfileBuilder::logos)
    /// - [`name`](BrandProfileBuilder::name)
    /// - [`socials`](BrandProfileBuilder::socials)
    pub fn build(self) -> Result<BrandProfile, BuildError> {
        Ok(BrandProfile {
            colors: self
                .colors
                .ok_or_else(|| BuildError::missing_field("colors"))?,
            description: self.description,
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            fonts: self
                .fonts
                .ok_or_else(|| BuildError::missing_field("fonts"))?,
            images: self
                .images
                .ok_or_else(|| BuildError::missing_field("images"))?,
            language: self.language,
            logos: self
                .logos
                .ok_or_else(|| BuildError::missing_field("logos"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            socials: self
                .socials
                .ok_or_else(|| BuildError::missing_field("socials"))?,
            tagline: self.tagline,
        })
    }
}
