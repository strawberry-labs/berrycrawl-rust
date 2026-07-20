# Reference
<details><summary><code>client.<a href="/src/client.rs">crawl</a>(request: CrawlDto) -> Result&lt;JobCreatedResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client
        .crawl(
            &CrawlDto {
                url: "https://example.com".to_string(),
                allow_external_links: None,
                allow_subdomains: None,
                crawl_entire_domain: None,
                deduplicate_similar_ur_ls: None,
                delay: None,
                exclude_paths: None,
                ignore_query_parameters: None,
                include_paths: None,
                limit: None,
                max_concurrency: None,
                max_discovery_depth: None,
                prompt: None,
                scrape_options: None,
                sitemap: None,
                webhook: None,
                zero_data_retention: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**allow_external_links:** `Option<bool>` — Whether to allow crawling external domains
    
</dd>
</dl>

<dl>
<dd>

**allow_subdomains:** `Option<bool>` — Whether to allow crawling subdomains
    
</dd>
</dl>

<dl>
<dd>

**crawl_entire_domain:** `Option<bool>` — Whether to crawl entire domain or just subtree
    
</dd>
</dl>

<dl>
<dd>

**deduplicate_similar_ur_ls:** `Option<bool>` — Deduplicate similar URLs using intelligent matching
    
</dd>
</dl>

<dl>
<dd>

**delay:** `Option<f64>` — Delay between page scrapes in milliseconds
    
</dd>
</dl>

<dl>
<dd>

**exclude_paths:** `Option<Vec<String>>` — Regex patterns to exclude paths
    
</dd>
</dl>

<dl>
<dd>

**ignore_query_parameters:** `Option<bool>` — Ignore query parameters when deduplicating URLs
    
</dd>
</dl>

<dl>
<dd>

**include_paths:** `Option<Vec<String>>` — Regex patterns to include paths
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<f64>` — Maximum number of pages to crawl
    
</dd>
</dl>

<dl>
<dd>

**max_concurrency:** `Option<f64>` — Maximum number of concurrent scrapes for this crawl
    
</dd>
</dl>

<dl>
<dd>

**max_discovery_depth:** `Option<f64>` — Maximum depth for URL discovery
    
</dd>
</dl>

<dl>
<dd>

**prompt:** `Option<String>` — Natural language instructions for crawl configuration
    
</dd>
</dl>

<dl>
<dd>

**scrape_options:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Scrape options to apply to each page. actions accepts at most 25 bounded browser actions. zeroDataRetention: true is currently rejected with 400 ZERO_DATA_RETENTION_NOT_SUPPORTED.
    
</dd>
</dl>

<dl>
<dd>

**sitemap:** `Option<CrawlDtoSitemap>` — Sitemap handling strategy
    
</dd>
</dl>

<dl>
<dd>

**url:** `String` — Starting URL to crawl
    
</dd>
</dl>

<dl>
<dd>

**webhook:** `Option<WebhookConfigDto>` — Webhook configuration
    
</dd>
</dl>

<dl>
<dd>

**zero_data_retention:** `Option<bool>` — Reserved for a future zero-data-retention mode. true is currently rejected with 400 ZERO_DATA_RETENTION_NOT_SUPPORTED; omit or use false.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">extract</a>(request: ExtractDto) -> Result&lt;JobCreatedResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client
        .extract(
            &ExtractDto {
                prompt: "Extract all product names, prices, and descriptions from these pages"
                    .to_string(),
                agent: None,
                enable_web_search: None,
                ignore_invalid_ur_ls: None,
                ignore_sitemap: None,
                include_subdomains: None,
                schema: None,
                scrape_options: None,
                show_sources: None,
                urls: None,
                webhook: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**agent:** `Option<AgentConfigDto>` — Agent configuration
    
</dd>
</dl>

<dl>
<dd>

**enable_web_search:** `Option<bool>` — Enable web search for URL discovery
    
</dd>
</dl>

<dl>
<dd>

**ignore_invalid_ur_ls:** `Option<bool>` — Ignore invalid URLs and process remaining valid ones
    
</dd>
</dl>

<dl>
<dd>

**ignore_sitemap:** `Option<bool>` — Ignore sitemap during URL discovery
    
</dd>
</dl>

<dl>
<dd>

**include_subdomains:** `Option<bool>` — Include subdomains in extraction
    
</dd>
</dl>

<dl>
<dd>

**prompt:** `String` — Natural language prompt describing what to extract. Maximum 16384 UTF-8 bytes.
    
</dd>
</dl>

<dl>
<dd>

**schema:** `Option<std::collections::HashMap<String, serde_json::Value>>` — JSON Schema for structured output. Serialized schema is limited to 65536 UTF-8 bytes.
    
</dd>
</dl>

<dl>
<dd>

**scrape_options:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Scrape options for each URL. zeroDataRetention: true is currently rejected with 400 ZERO_DATA_RETENTION_NOT_SUPPORTED.
    
</dd>
</dl>

<dl>
<dd>

**show_sources:** `Option<bool>` — Include source citations in response
    
</dd>
</dl>

<dl>
<dd>

**urls:** `Option<Vec<String>>` — 1–20 unique public HTTP(S) URLs. May be omitted only when enableWebSearch is true. Each URL is limited to 2048 UTF-8 bytes.
    
</dd>
</dl>

<dl>
<dd>

**webhook:** `Option<ExtractWebhookConfigDto>` — Webhook configuration
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">map</a>(request: MapDto) -> Result&lt;MapResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client
        .map(
            &MapDto {
                url: "https://example.com".to_string(),
                ignore_query_parameters: None,
                include_subdomains: None,
                limit: None,
                location: None,
                search: None,
                sitemap: None,
                timeout: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ignore_query_parameters:** `Option<bool>` — Ignore query parameters when discovering URLs
    
</dd>
</dl>

<dl>
<dd>

**include_subdomains:** `Option<bool>` — Include subdomains in the map
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<f64>` — Maximum number of URLs to return
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<LocationDto>` — Location configuration
    
</dd>
</dl>

<dl>
<dd>

**search:** `Option<String>` — Search filter for URLs
    
</dd>
</dl>

<dl>
<dd>

**sitemap:** `Option<MapDtoSitemap>` — How to handle sitemaps
    
</dd>
</dl>

<dl>
<dd>

**timeout:** `Option<f64>` — Timeout for map operation in milliseconds
    
</dd>
</dl>

<dl>
<dd>

**url:** `String` — URL to map
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">parse</a>(request: ParseDto) -> Result&lt;ScrapeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client
        .parse(
            &ParseDto {
                url: "https://example.com/report.pdf".to_string(),
                timeout: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**timeout:** `Option<f64>` 
    
</dd>
</dl>

<dl>
<dd>

**url:** `String` — Public PDF, Word document, or spreadsheet URL
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">scrape</a>(request: ScrapeDto) -> Result&lt;ScrapeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client
        .scrape(
            &ScrapeDto {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**actions:** `Option<Vec<ActionDto>>` — Browser actions to execute
    
</dd>
</dl>

<dl>
<dd>

**block_ads:** `Option<bool>` — Enable ad-blocking and cookie popup blocking
    
</dd>
</dl>

<dl>
<dd>

**domain:** `Option<String>` — Domain to scrape. Normalized to https://domain when url is omitted.
    
</dd>
</dl>

<dl>
<dd>

**exclude_tags:** `Option<Vec<String>>` — CSS selectors to exclude
    
</dd>
</dl>

<dl>
<dd>

**extraction_schema:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Schema for structured data extraction (used with json format)
    
</dd>
</dl>

<dl>
<dd>

**formats:** `Option<Vec<ScrapeDtoFormatsItem>>` — Output formats - can be simple strings or format objects with options
    
</dd>
</dl>

<dl>
<dd>

**headers:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Custom headers
    
</dd>
</dl>

<dl>
<dd>

**include_tags:** `Option<Vec<String>>` — CSS selectors to include
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<LocationDto>` — Location settings
    
</dd>
</dl>

<dl>
<dd>

**max_age:** `Option<f64>` — Cache max age in milliseconds
    
</dd>
</dl>

<dl>
<dd>

**mobile:** `Option<bool>` — Emulate mobile device for scraping
    
</dd>
</dl>

<dl>
<dd>

**only_main_content:** `Option<bool>` — Extract only main content
    
</dd>
</dl>

<dl>
<dd>

**proxy:** `Option<ScrapeDtoProxy>` — Proxy mode. auto starts direct and escalates only when blocked. basic is an alias for none.
    
</dd>
</dl>

<dl>
<dd>

**remove_base64images:** `Option<bool>` — Remove base64 images from output (keeps alt text)
    
</dd>
</dl>

<dl>
<dd>

**screenshot_as_base64:** `Option<bool>` — Return screenshot/PDF output inline as a base64 data URL instead of an uploaded CDN URL. Default false (a CDN URL is returned).
    
</dd>
</dl>

<dl>
<dd>

**skip_tls_verification:** `Option<bool>` — Skip TLS certificate verification
    
</dd>
</dl>

<dl>
<dd>

**store_in_cache:** `Option<bool>` — Store result in cache
    
</dd>
</dl>

<dl>
<dd>

**timeout:** `Option<f64>` — Request timeout in milliseconds
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — URL to scrape. Either url or domain is required.
    
</dd>
</dl>

<dl>
<dd>

**wait_for:** `Option<f64>` — Wait time before scraping (ms)
    
</dd>
</dl>

<dl>
<dd>

**zero_data_retention:** `Option<bool>` — Reserved for a future zero-data-retention mode. true is currently rejected with 400 ZERO_DATA_RETENTION_NOT_SUPPORTED; omit or use false.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">screenshot</a>(request: ScreenshotDto) -> Result&lt;ScrapeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cookie banners, blocking overlays, chat widgets, and lazy loading are handled by default. Every cleanup pass can be controlled per request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client
        .screenshot(
            &ScreenshotDto {
                url: "https://example.com".to_string(),
                block_ads: None,
                click_selector: None,
                clip: None,
                color_scheme: None,
                cookies: None,
                delay: None,
                device: None,
                disable_animations: None,
                format: None,
                full_page: None,
                full_page_algorithm: None,
                headers: None,
                hide_fixed_elements: None,
                hide_selectors: None,
                location: None,
                mask_color: None,
                mask_selectors: None,
                max_height: None,
                omit_background: None,
                proxy: None,
                quality: None,
                reduced_motion: None,
                remove_chat_widgets: None,
                remove_cookie_banners: None,
                remove_overlays: None,
                response_format: None,
                scale: None,
                scroll_delay: None,
                scroll_page: None,
                selector: None,
                styles: None,
                timeout: None,
                viewport: None,
                wait_for_selector: None,
                wait_until: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**block_ads:** `Option<bool>` — Block common advertising and analytics requests
    
</dd>
</dl>

<dl>
<dd>

**click_selector:** `Option<String>` — Click this CSS selector before capture
    
</dd>
</dl>

<dl>
<dd>

**clip:** `Option<ScreenshotClipDto>` — Capture an exact pixel rectangle instead of the page
    
</dd>
</dl>

<dl>
<dd>

**color_scheme:** `Option<ScreenshotDtoColorScheme>` 
    
</dd>
</dl>

<dl>
<dd>

**cookies:** `Option<Vec<ScreenshotCookieDto>>` — Cookies to set before loading the page
    
</dd>
</dl>

<dl>
<dd>

**delay:** `Option<f64>` — Extra settling time after the page is ready, in milliseconds
    
</dd>
</dl>

<dl>
<dd>

**device:** `Option<ScreenshotDtoDevice>` — Named viewport preset
    
</dd>
</dl>

<dl>
<dd>

**disable_animations:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<ScreenshotDtoFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**full_page:** `Option<bool>` — Capture the complete page instead of only the viewport
    
</dd>
</dl>

<dl>
<dd>

**full_page_algorithm:** `Option<ScreenshotDtoFullPageAlgorithm>` — auto uses native capture for normal pages and stitched slices for very tall pages
    
</dd>
</dl>

<dl>
<dd>

**headers:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Headers sent while loading the page
    
</dd>
</dl>

<dl>
<dd>

**hide_fixed_elements:** `Option<bool>` — Show fixed/sticky UI once instead of repeating it in stitched captures
    
</dd>
</dl>

<dl>
<dd>

**hide_selectors:** `Option<Vec<String>>` — Hide matching elements before capture
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<ScreenshotLocationDto>` 
    
</dd>
</dl>

<dl>
<dd>

**mask_color:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**mask_selectors:** `Option<Vec<String>>` — Cover matching elements with a solid privacy mask
    
</dd>
</dl>

<dl>
<dd>

**max_height:** `Option<f64>` — Maximum full-page height. Prevents endless captures on infinite pages.
    
</dd>
</dl>

<dl>
<dd>

**omit_background:** `Option<bool>` — Use a transparent background where the page allows it
    
</dd>
</dl>

<dl>
<dd>

**proxy:** `Option<ScreenshotDtoProxy>` — Proxy mode
    
</dd>
</dl>

<dl>
<dd>

**quality:** `Option<f64>` — JPEG or WebP quality
    
</dd>
</dl>

<dl>
<dd>

**reduced_motion:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**remove_chat_widgets:** `Option<bool>` — Hide common support and chat widgets
    
</dd>
</dl>

<dl>
<dd>

**remove_cookie_banners:** `Option<bool>` — Accept known consent dialogs, hide remaining cookie banners, and restore page scrolling
    
</dd>
</dl>

<dl>
<dd>

**remove_overlays:** `Option<bool>` — Remove newsletter gates, modal backdrops, and blocking overlays
    
</dd>
</dl>

<dl>
<dd>

**response_format:** `Option<ScreenshotDtoResponseFormat>` — Return a CDN URL or an inline data URL
    
</dd>
</dl>

<dl>
<dd>

**scale:** `Option<ScreenshotDtoScale>` — Capture at CSS pixel size or the emulated device pixel ratio
    
</dd>
</dl>

<dl>
<dd>

**scroll_delay:** `Option<f64>` — Pause between lazy-load scroll steps, in milliseconds
    
</dd>
</dl>

<dl>
<dd>

**scroll_page:** `Option<bool>` — Scroll through the page first so lazy content is loaded
    
</dd>
</dl>

<dl>
<dd>

**selector:** `Option<String>` — Capture one element instead of the page
    
</dd>
</dl>

<dl>
<dd>

**styles:** `Option<Vec<String>>` — CSS rules to apply before capture
    
</dd>
</dl>

<dl>
<dd>

**timeout:** `Option<f64>` 
    
</dd>
</dl>

<dl>
<dd>

**url:** `String` — Public webpage URL to capture
    
</dd>
</dl>

<dl>
<dd>

**viewport:** `Option<ScreenshotViewportDto>` — Custom viewport. Overrides the named device dimensions.
    
</dd>
</dl>

<dl>
<dd>

**wait_for_selector:** `Option<String>` — Wait for this CSS selector before capture
    
</dd>
</dl>

<dl>
<dd>

**wait_until:** `Option<ScreenshotDtoWaitUntil>` — Page readiness milestone used before capture
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">search</a>(request: SearchDto) -> Result&lt;SearchResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client
        .search(
            &SearchDto {
                query: "machine learning tutorials".to_string(),
                categories: None,
                country: None,
                limit: None,
                location: None,
                sources: None,
                tbs: None,
                timeout: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**categories:** `Option<Vec<String>>` — Category filters
    
</dd>
</dl>

<dl>
<dd>

**country:** `Option<String>` — Country code
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<f64>` — Maximum number of results
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<String>` — Location for geo-targeting
    
</dd>
</dl>

<dl>
<dd>

**query:** `String` — Search query
    
</dd>
</dl>

<dl>
<dd>

**sources:** `Option<Vec<String>>` — Source types to search
    
</dd>
</dl>

<dl>
<dd>

**tbs:** `Option<String>` — Time-based filter (e.g., qdr:d for past day)
    
</dd>
</dl>

<dl>
<dd>

**timeout:** `Option<f64>` — Timeout for search operation in milliseconds
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Account
<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">get</a>() -> Result&lt;AccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client.account.get(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Brand
<details><summary><code>client.brand.<a href="/src/api/resources/brand/client.rs">retrieve</a>(request: BrandDto) -> Result&lt;BrandResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send one website URL. BerryCrawl returns the brand identity found on that site.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client
        .brand
        .retrieve(
            &BrandDto {
                url: "https://stripe.com".to_string(),
                refresh: None,
                timeout: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**refresh:** `Option<bool>` — Ignore a cached profile and fetch the website again
    
</dd>
</dl>

<dl>
<dd>

**timeout:** `Option<f64>` — Maximum time to spend building the profile, in milliseconds
    
</dd>
</dl>

<dl>
<dd>

**url:** `String` — The public website whose brand profile should be extracted
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Jobs
<details><summary><code>client.jobs.<a href="/src/api/resources/jobs/client.rs">list</a>(type_: Option&lt;Option&lt;ListJobsRequestType&gt;&gt;, status: Option&lt;Option&lt;ListJobsRequestStatus&gt;&gt;, page: Option&lt;Option&lt;f64&gt;&gt;, limit: Option&lt;Option&lt;f64&gt;&gt;) -> Result&lt;ListJobsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client
        .jobs
        .list(
            &ListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**type_:** `Option<ListJobsRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListJobsRequestStatus>` 
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<f64>` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<f64>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.jobs.<a href="/src/api/resources/jobs/client.rs">get</a>(id: String, page: Option&lt;Option&lt;f64&gt;&gt;, limit: Option&lt;Option&lt;f64&gt;&gt;) -> Result&lt;JobResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client
        .jobs
        .get(
            &"id".to_string(),
            &GetQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<f64>` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<f64>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.jobs.<a href="/src/api/resources/jobs/client.rs">cancel</a>(id: String) -> Result&lt;CancelJobResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client.jobs.cancel(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Webhooks
<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">list</a>() -> Result&lt;ListWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client.webhooks.list(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">create</a>(request: CreateWebhookDto) -> Result&lt;WebhookResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client
        .webhooks
        .create(
            &CreateWebhookDto {
                events: vec!["crawl.completed".to_string(), "extract.failed".to_string()],
                url: "https://api.example.com/webhooks".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**events:** `Vec<String>` — Event types to subscribe to
    
</dd>
</dl>

<dl>
<dd>

**url:** `String` — Webhook URL to send events to
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">get</a>(id: String) -> Result&lt;WebhookResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client.webhooks.get(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">delete</a>(id: String) -> Result&lt;MessageResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client.webhooks.delete(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">update</a>(id: String, request: UpdateWebhookDto) -> Result&lt;WebhookResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client
        .webhooks
        .update(
            &"id".to_string(),
            &UpdateWebhookDto {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**events:** `Option<Vec<String>>` — Event types to subscribe to
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` — Enable or disable webhook
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — Webhook URL to send events to
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">test</a>(id: String, request: TestWebhookDto) -> Result&lt;TestWebhookResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Berrycrawl::new(config).expect("Failed to build client");
    client
        .webhooks
        .test(
            &"id".to_string(),
            &TestWebhookDto {
                event: "crawl.completed".to_string(),
                payload: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**event:** `String` — Event type to test
    
</dd>
</dl>

<dl>
<dd>

**payload:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Optional custom payload for testing
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

