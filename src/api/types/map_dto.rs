pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MapDto {
    /// Ignore query parameters when discovering URLs
    #[serde(rename = "ignoreQueryParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_query_parameters: Option<bool>,
    /// Include subdomains in the map
    #[serde(rename = "includeSubdomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    /// Maximum number of URLs to return
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub limit: Option<f64>,
    /// Location configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationDto>,
    /// Search filter for URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// How to handle sitemaps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sitemap: Option<MapDtoSitemap>,
    /// Timeout for map operation in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub timeout: Option<f64>,
    /// URL to map
    #[serde(default)]
    pub url: String,
}

impl MapDto {
    pub fn builder() -> MapDtoBuilder {
        <MapDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MapDtoBuilder {
    ignore_query_parameters: Option<bool>,
    include_subdomains: Option<bool>,
    limit: Option<f64>,
    location: Option<LocationDto>,
    search: Option<String>,
    sitemap: Option<MapDtoSitemap>,
    timeout: Option<f64>,
    url: Option<String>,
}

impl MapDtoBuilder {
    pub fn ignore_query_parameters(mut self, value: bool) -> Self {
        self.ignore_query_parameters = Some(value);
        self
    }

    pub fn include_subdomains(mut self, value: bool) -> Self {
        self.include_subdomains = Some(value);
        self
    }

    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn location(mut self, value: LocationDto) -> Self {
        self.location = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn sitemap(mut self, value: MapDtoSitemap) -> Self {
        self.sitemap = Some(value);
        self
    }

    pub fn timeout(mut self, value: f64) -> Self {
        self.timeout = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MapDto`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](MapDtoBuilder::url)
    pub fn build(self) -> Result<MapDto, BuildError> {
        Ok(MapDto {
            ignore_query_parameters: self.ignore_query_parameters,
            include_subdomains: self.include_subdomains,
            limit: self.limit,
            location: self.location,
            search: self.search,
            sitemap: self.sitemap,
            timeout: self.timeout,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
