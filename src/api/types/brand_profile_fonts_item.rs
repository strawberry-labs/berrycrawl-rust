pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrandProfileFontsItem {
    #[serde(default)]
    pub family: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<String>>,
}

impl BrandProfileFontsItem {
    pub fn builder() -> BrandProfileFontsItemBuilder {
        <BrandProfileFontsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandProfileFontsItemBuilder {
    family: Option<String>,
    weights: Option<Vec<String>>,
}

impl BrandProfileFontsItemBuilder {
    pub fn family(mut self, value: impl Into<String>) -> Self {
        self.family = Some(value.into());
        self
    }

    pub fn weights(mut self, value: Vec<String>) -> Self {
        self.weights = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BrandProfileFontsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`family`](BrandProfileFontsItemBuilder::family)
    pub fn build(self) -> Result<BrandProfileFontsItem, BuildError> {
        Ok(BrandProfileFontsItem {
            family: self
                .family
                .ok_or_else(|| BuildError::missing_field("family"))?,
            weights: self.weights,
        })
    }
}
