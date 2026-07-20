pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrandDesignSystemComponents {
    #[serde(rename = "buttonPrimary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_primary: Option<BrandComponentStyle>,
    #[serde(rename = "buttonSecondary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_secondary: Option<BrandComponentStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<BrandComponentStyle>,
}

impl BrandDesignSystemComponents {
    pub fn builder() -> BrandDesignSystemComponentsBuilder {
        <BrandDesignSystemComponentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandDesignSystemComponentsBuilder {
    button_primary: Option<BrandComponentStyle>,
    button_secondary: Option<BrandComponentStyle>,
    input: Option<BrandComponentStyle>,
}

impl BrandDesignSystemComponentsBuilder {
    pub fn button_primary(mut self, value: BrandComponentStyle) -> Self {
        self.button_primary = Some(value);
        self
    }

    pub fn button_secondary(mut self, value: BrandComponentStyle) -> Self {
        self.button_secondary = Some(value);
        self
    }

    pub fn input(mut self, value: BrandComponentStyle) -> Self {
        self.input = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BrandDesignSystemComponents`].
    pub fn build(self) -> Result<BrandDesignSystemComponents, BuildError> {
        Ok(BrandDesignSystemComponents {
            button_primary: self.button_primary,
            button_secondary: self.button_secondary,
            input: self.input,
        })
    }
}
