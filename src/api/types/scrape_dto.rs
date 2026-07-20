pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScrapeDto {
    /// Browser actions to execute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<ActionDto>>,
    /// Enable ad-blocking and cookie popup blocking
    #[serde(rename = "blockAds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ads: Option<bool>,
    /// Domain to scrape. Normalized to https://domain when url is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// CSS selectors to exclude
    #[serde(rename = "excludeTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_tags: Option<Vec<String>>,
    /// Schema for structured data extraction (used with json format)
    #[serde(rename = "extractionSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extraction_schema: Option<HashMap<String, serde_json::Value>>,
    /// Output formats - can be simple strings or format objects with options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formats: Option<Vec<ScrapeDtoFormatsItem>>,
    /// Custom headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, serde_json::Value>>,
    /// CSS selectors to include
    #[serde(rename = "includeTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_tags: Option<Vec<String>>,
    /// Location settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationDto>,
    /// Cache max age in milliseconds
    #[serde(rename = "maxAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_age: Option<f64>,
    /// Emulate mobile device for scraping
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<bool>,
    /// Extract only main content
    #[serde(rename = "onlyMainContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_main_content: Option<bool>,
    /// Proxy mode. auto starts direct and escalates only when blocked. basic is an alias for none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<ScrapeDtoProxy>,
    /// Remove base64 images from output (keeps alt text)
    #[serde(rename = "removeBase64Images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_base64images: Option<bool>,
    /// Return screenshot/PDF output inline as a base64 data URL instead of an uploaded CDN URL. Default false (a CDN URL is returned).
    #[serde(rename = "screenshotAsBase64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot_as_base64: Option<bool>,
    /// Skip TLS certificate verification
    #[serde(rename = "skipTlsVerification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_tls_verification: Option<bool>,
    /// Store result in cache
    #[serde(rename = "storeInCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_in_cache: Option<bool>,
    /// Request timeout in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub timeout: Option<f64>,
    /// URL to scrape. Either url or domain is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Wait time before scraping (ms)
    #[serde(rename = "waitFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wait_for: Option<f64>,
    /// Reserved for a future zero-data-retention mode. true is currently rejected with 400 ZERO_DATA_RETENTION_NOT_SUPPORTED; omit or use false.
    #[serde(rename = "zeroDataRetention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_data_retention: Option<bool>,
}

impl ScrapeDto {
    pub fn builder() -> ScrapeDtoBuilder {
        <ScrapeDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScrapeDtoBuilder {
    actions: Option<Vec<ActionDto>>,
    block_ads: Option<bool>,
    domain: Option<String>,
    exclude_tags: Option<Vec<String>>,
    extraction_schema: Option<HashMap<String, serde_json::Value>>,
    formats: Option<Vec<ScrapeDtoFormatsItem>>,
    headers: Option<HashMap<String, serde_json::Value>>,
    include_tags: Option<Vec<String>>,
    location: Option<LocationDto>,
    max_age: Option<f64>,
    mobile: Option<bool>,
    only_main_content: Option<bool>,
    proxy: Option<ScrapeDtoProxy>,
    remove_base64images: Option<bool>,
    screenshot_as_base64: Option<bool>,
    skip_tls_verification: Option<bool>,
    store_in_cache: Option<bool>,
    timeout: Option<f64>,
    url: Option<String>,
    wait_for: Option<f64>,
    zero_data_retention: Option<bool>,
}

impl ScrapeDtoBuilder {
    pub fn actions(mut self, value: Vec<ActionDto>) -> Self {
        self.actions = Some(value);
        self
    }

    pub fn block_ads(mut self, value: bool) -> Self {
        self.block_ads = Some(value);
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn exclude_tags(mut self, value: Vec<String>) -> Self {
        self.exclude_tags = Some(value);
        self
    }

    pub fn extraction_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.extraction_schema = Some(value);
        self
    }

    pub fn formats(mut self, value: Vec<ScrapeDtoFormatsItem>) -> Self {
        self.formats = Some(value);
        self
    }

    pub fn headers(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn include_tags(mut self, value: Vec<String>) -> Self {
        self.include_tags = Some(value);
        self
    }

    pub fn location(mut self, value: LocationDto) -> Self {
        self.location = Some(value);
        self
    }

    pub fn max_age(mut self, value: f64) -> Self {
        self.max_age = Some(value);
        self
    }

    pub fn mobile(mut self, value: bool) -> Self {
        self.mobile = Some(value);
        self
    }

    pub fn only_main_content(mut self, value: bool) -> Self {
        self.only_main_content = Some(value);
        self
    }

    pub fn proxy(mut self, value: ScrapeDtoProxy) -> Self {
        self.proxy = Some(value);
        self
    }

    pub fn remove_base64images(mut self, value: bool) -> Self {
        self.remove_base64images = Some(value);
        self
    }

    pub fn screenshot_as_base64(mut self, value: bool) -> Self {
        self.screenshot_as_base64 = Some(value);
        self
    }

    pub fn skip_tls_verification(mut self, value: bool) -> Self {
        self.skip_tls_verification = Some(value);
        self
    }

    pub fn store_in_cache(mut self, value: bool) -> Self {
        self.store_in_cache = Some(value);
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

    pub fn wait_for(mut self, value: f64) -> Self {
        self.wait_for = Some(value);
        self
    }

    pub fn zero_data_retention(mut self, value: bool) -> Self {
        self.zero_data_retention = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScrapeDto`].
    pub fn build(self) -> Result<ScrapeDto, BuildError> {
        Ok(ScrapeDto {
            actions: self.actions,
            block_ads: self.block_ads,
            domain: self.domain,
            exclude_tags: self.exclude_tags,
            extraction_schema: self.extraction_schema,
            formats: self.formats,
            headers: self.headers,
            include_tags: self.include_tags,
            location: self.location,
            max_age: self.max_age,
            mobile: self.mobile,
            only_main_content: self.only_main_content,
            proxy: self.proxy,
            remove_base64images: self.remove_base64images,
            screenshot_as_base64: self.screenshot_as_base64,
            skip_tls_verification: self.skip_tls_verification,
            store_in_cache: self.store_in_cache,
            timeout: self.timeout,
            url: self.url,
            wait_for: self.wait_for,
            zero_data_retention: self.zero_data_retention,
        })
    }
}
