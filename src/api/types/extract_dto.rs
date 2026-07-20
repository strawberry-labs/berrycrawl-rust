pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExtractDto {
    /// Agent configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentConfigDto>,
    /// Enable web search for URL discovery
    #[serde(rename = "enableWebSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_web_search: Option<bool>,
    /// Ignore invalid URLs and process remaining valid ones
    #[serde(rename = "ignoreInvalidURLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_invalid_ur_ls: Option<bool>,
    /// Ignore sitemap during URL discovery
    #[serde(rename = "ignoreSitemap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_sitemap: Option<bool>,
    /// Include subdomains in extraction
    #[serde(rename = "includeSubdomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    /// Natural language prompt describing what to extract. Maximum 16384 UTF-8 bytes.
    #[serde(default)]
    pub prompt: String,
    /// JSON Schema for structured output. Serialized schema is limited to 65536 UTF-8 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<HashMap<String, serde_json::Value>>,
    /// Scrape options for each URL. zeroDataRetention: true is currently rejected with 400 ZERO_DATA_RETENTION_NOT_SUPPORTED.
    #[serde(rename = "scrapeOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrape_options: Option<HashMap<String, serde_json::Value>>,
    /// Include source citations in response
    #[serde(rename = "showSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_sources: Option<bool>,
    /// 1–20 unique public HTTP(S) URLs. May be omitted only when enableWebSearch is true. Each URL is limited to 2048 UTF-8 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
    /// Webhook configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<ExtractWebhookConfigDto>,
}

impl ExtractDto {
    pub fn builder() -> ExtractDtoBuilder {
        <ExtractDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExtractDtoBuilder {
    agent: Option<AgentConfigDto>,
    enable_web_search: Option<bool>,
    ignore_invalid_ur_ls: Option<bool>,
    ignore_sitemap: Option<bool>,
    include_subdomains: Option<bool>,
    prompt: Option<String>,
    schema: Option<HashMap<String, serde_json::Value>>,
    scrape_options: Option<HashMap<String, serde_json::Value>>,
    show_sources: Option<bool>,
    urls: Option<Vec<String>>,
    webhook: Option<ExtractWebhookConfigDto>,
}

impl ExtractDtoBuilder {
    pub fn agent(mut self, value: AgentConfigDto) -> Self {
        self.agent = Some(value);
        self
    }

    pub fn enable_web_search(mut self, value: bool) -> Self {
        self.enable_web_search = Some(value);
        self
    }

    pub fn ignore_invalid_ur_ls(mut self, value: bool) -> Self {
        self.ignore_invalid_ur_ls = Some(value);
        self
    }

    pub fn ignore_sitemap(mut self, value: bool) -> Self {
        self.ignore_sitemap = Some(value);
        self
    }

    pub fn include_subdomains(mut self, value: bool) -> Self {
        self.include_subdomains = Some(value);
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.schema = Some(value);
        self
    }

    pub fn scrape_options(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.scrape_options = Some(value);
        self
    }

    pub fn show_sources(mut self, value: bool) -> Self {
        self.show_sources = Some(value);
        self
    }

    pub fn urls(mut self, value: Vec<String>) -> Self {
        self.urls = Some(value);
        self
    }

    pub fn webhook(mut self, value: ExtractWebhookConfigDto) -> Self {
        self.webhook = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExtractDto`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](ExtractDtoBuilder::prompt)
    pub fn build(self) -> Result<ExtractDto, BuildError> {
        Ok(ExtractDto {
            agent: self.agent,
            enable_web_search: self.enable_web_search,
            ignore_invalid_ur_ls: self.ignore_invalid_ur_ls,
            ignore_sitemap: self.ignore_sitemap,
            include_subdomains: self.include_subdomains,
            prompt: self
                .prompt
                .ok_or_else(|| BuildError::missing_field("prompt"))?,
            schema: self.schema,
            scrape_options: self.scrape_options,
            show_sources: self.show_sources,
            urls: self.urls,
            webhook: self.webhook,
        })
    }
}
