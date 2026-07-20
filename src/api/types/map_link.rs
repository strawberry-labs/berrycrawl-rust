pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MapLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub url: String,
}

impl MapLink {
    pub fn builder() -> MapLinkBuilder {
        <MapLinkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MapLinkBuilder {
    description: Option<String>,
    title: Option<String>,
    url: Option<String>,
}

impl MapLinkBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MapLink`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](MapLinkBuilder::url)
    pub fn build(self) -> Result<MapLink, BuildError> {
        Ok(MapLink {
            description: self.description,
            title: self.title,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
