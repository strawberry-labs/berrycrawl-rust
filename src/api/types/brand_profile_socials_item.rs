pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrandProfileSocialsItem {
    #[serde(default)]
    pub network: String,
    #[serde(default)]
    pub url: String,
}

impl BrandProfileSocialsItem {
    pub fn builder() -> BrandProfileSocialsItemBuilder {
        <BrandProfileSocialsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandProfileSocialsItemBuilder {
    network: Option<String>,
    url: Option<String>,
}

impl BrandProfileSocialsItemBuilder {
    pub fn network(mut self, value: impl Into<String>) -> Self {
        self.network = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrandProfileSocialsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`network`](BrandProfileSocialsItemBuilder::network)
    /// - [`url`](BrandProfileSocialsItemBuilder::url)
    pub fn build(self) -> Result<BrandProfileSocialsItem, BuildError> {
        Ok(BrandProfileSocialsItem {
            network: self
                .network
                .ok_or_else(|| BuildError::missing_field("network"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
