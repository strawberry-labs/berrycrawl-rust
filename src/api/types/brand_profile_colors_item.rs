pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrandProfileColorsItem {
    #[serde(default)]
    pub hex: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl BrandProfileColorsItem {
    pub fn builder() -> BrandProfileColorsItemBuilder {
        <BrandProfileColorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandProfileColorsItemBuilder {
    hex: Option<String>,
    name: Option<String>,
}

impl BrandProfileColorsItemBuilder {
    pub fn hex(mut self, value: impl Into<String>) -> Self {
        self.hex = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrandProfileColorsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`hex`](BrandProfileColorsItemBuilder::hex)
    pub fn build(self) -> Result<BrandProfileColorsItem, BuildError> {
        Ok(BrandProfileColorsItem {
            hex: self.hex.ok_or_else(|| BuildError::missing_field("hex"))?,
            name: self.name,
        })
    }
}
