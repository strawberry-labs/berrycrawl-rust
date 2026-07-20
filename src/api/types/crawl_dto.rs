pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CrawlDto {
    /// Whether to allow crawling external domains
    #[serde(rename = "allowExternalLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_external_links: Option<bool>,
    /// Whether to allow crawling subdomains
    #[serde(rename = "allowSubdomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_subdomains: Option<bool>,
    /// Whether to crawl entire domain or just subtree
    #[serde(rename = "crawlEntireDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_entire_domain: Option<bool>,
    /// Deduplicate similar URLs using intelligent matching
    #[serde(rename = "deduplicateSimilarURLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduplicate_similar_ur_ls: Option<bool>,
    /// Delay between page scrapes in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub delay: Option<f64>,
    /// Regex patterns to exclude paths
    #[serde(rename = "excludePaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_paths: Option<Vec<String>>,
    /// Ignore query parameters when deduplicating URLs
    #[serde(rename = "ignoreQueryParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_query_parameters: Option<bool>,
    /// Regex patterns to include paths
    #[serde(rename = "includePaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_paths: Option<Vec<String>>,
    /// Maximum number of pages to crawl
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub limit: Option<f64>,
    /// Maximum number of concurrent scrapes for this crawl
    #[serde(rename = "maxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_concurrency: Option<f64>,
    /// Maximum depth for URL discovery
    #[serde(rename = "maxDiscoveryDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_discovery_depth: Option<f64>,
    /// Natural language instructions for crawl configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Scrape options to apply to each page. actions accepts at most 25 bounded browser actions. zeroDataRetention: true is currently rejected with 400 ZERO_DATA_RETENTION_NOT_SUPPORTED.
    #[serde(rename = "scrapeOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrape_options: Option<HashMap<String, serde_json::Value>>,
    /// Sitemap handling strategy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sitemap: Option<CrawlDtoSitemap>,
    /// Starting URL to crawl
    #[serde(default)]
    pub url: String,
    /// Webhook configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WebhookConfigDto>,
    /// Reserved for a future zero-data-retention mode. true is currently rejected with 400 ZERO_DATA_RETENTION_NOT_SUPPORTED; omit or use false.
    #[serde(rename = "zeroDataRetention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_data_retention: Option<bool>,
}

impl CrawlDto {
    pub fn builder() -> CrawlDtoBuilder {
        <CrawlDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CrawlDtoBuilder {
    allow_external_links: Option<bool>,
    allow_subdomains: Option<bool>,
    crawl_entire_domain: Option<bool>,
    deduplicate_similar_ur_ls: Option<bool>,
    delay: Option<f64>,
    exclude_paths: Option<Vec<String>>,
    ignore_query_parameters: Option<bool>,
    include_paths: Option<Vec<String>>,
    limit: Option<f64>,
    max_concurrency: Option<f64>,
    max_discovery_depth: Option<f64>,
    prompt: Option<String>,
    scrape_options: Option<HashMap<String, serde_json::Value>>,
    sitemap: Option<CrawlDtoSitemap>,
    url: Option<String>,
    webhook: Option<WebhookConfigDto>,
    zero_data_retention: Option<bool>,
}

impl CrawlDtoBuilder {
    pub fn allow_external_links(mut self, value: bool) -> Self {
        self.allow_external_links = Some(value);
        self
    }

    pub fn allow_subdomains(mut self, value: bool) -> Self {
        self.allow_subdomains = Some(value);
        self
    }

    pub fn crawl_entire_domain(mut self, value: bool) -> Self {
        self.crawl_entire_domain = Some(value);
        self
    }

    pub fn deduplicate_similar_ur_ls(mut self, value: bool) -> Self {
        self.deduplicate_similar_ur_ls = Some(value);
        self
    }

    pub fn delay(mut self, value: f64) -> Self {
        self.delay = Some(value);
        self
    }

    pub fn exclude_paths(mut self, value: Vec<String>) -> Self {
        self.exclude_paths = Some(value);
        self
    }

    pub fn ignore_query_parameters(mut self, value: bool) -> Self {
        self.ignore_query_parameters = Some(value);
        self
    }

    pub fn include_paths(mut self, value: Vec<String>) -> Self {
        self.include_paths = Some(value);
        self
    }

    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn max_concurrency(mut self, value: f64) -> Self {
        self.max_concurrency = Some(value);
        self
    }

    pub fn max_discovery_depth(mut self, value: f64) -> Self {
        self.max_discovery_depth = Some(value);
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn scrape_options(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.scrape_options = Some(value);
        self
    }

    pub fn sitemap(mut self, value: CrawlDtoSitemap) -> Self {
        self.sitemap = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn webhook(mut self, value: WebhookConfigDto) -> Self {
        self.webhook = Some(value);
        self
    }

    pub fn zero_data_retention(mut self, value: bool) -> Self {
        self.zero_data_retention = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CrawlDto`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](CrawlDtoBuilder::url)
    pub fn build(self) -> Result<CrawlDto, BuildError> {
        Ok(CrawlDto {
            allow_external_links: self.allow_external_links,
            allow_subdomains: self.allow_subdomains,
            crawl_entire_domain: self.crawl_entire_domain,
            deduplicate_similar_ur_ls: self.deduplicate_similar_ur_ls,
            delay: self.delay,
            exclude_paths: self.exclude_paths,
            ignore_query_parameters: self.ignore_query_parameters,
            include_paths: self.include_paths,
            limit: self.limit,
            max_concurrency: self.max_concurrency,
            max_discovery_depth: self.max_discovery_depth,
            prompt: self.prompt,
            scrape_options: self.scrape_options,
            sitemap: self.sitemap,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            webhook: self.webhook,
            zero_data_retention: self.zero_data_retention,
        })
    }
}
