# Berrycrawl Rust SDK

[![crates.io shield](https://img.shields.io/crates/v/berrycrawl)](https://crates.io/crates/berrycrawl)

The official Rust SDK for scraping, crawling, searching, mapping, structured extraction, screenshots, and brand profiles.

[Documentation](https://docs.berrycrawl.com) · [Dashboard](https://app.berrycrawl.com) · [GitHub](https://github.com/strawberry-labs/berrycrawl-rust)

## Table of Contents

- [Installation](#installation)
- [Reference](#reference)
- [Usage](#usage)
- [Environments](#environments)
- [Errors](#errors)
- [Request Types](#request-types)
- [Advanced](#advanced)
  - [Retries](#retries)
  - [Timeouts](#timeouts)
  - [Additional Headers](#additional-headers)
  - [Additional Query String Parameters](#additional-query-string-parameters)
- [Contributing](#contributing)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
berrycrawl = "0.1.0"
```

Or install via cargo:

```sh
cargo add berrycrawl
```

## Reference

A full reference for this library is available [here](./reference.md).

## Usage

Set `BERRYCRAWL_API_KEY` to an API key from the [Berrycrawl dashboard](https://app.berrycrawl.com).

```rust
use berrycrawl::prelude::*;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let client = Berrycrawl::new(ClientConfig {
        token: std::env::var("BERRYCRAWL_API_KEY").ok(),
        ..Default::default()
    }).expect("valid Berrycrawl configuration");

    let page = client.scrape(
        &ScrapeDto {
            url: Some("https://example.com/pricing".to_string()),
            ..Default::default()
        },
        None,
    ).await?;

    println!("{:?}", page);
    Ok(())
}
```

### Crawl a website

```rust
let job = client.crawl(
    &CrawlDto {
        url: "https://example.com/docs".to_string(),
        limit: Some(50.0),
        ..Default::default()
    },
    None,
).await?;
```

### Retrieve a brand profile

```rust
let brand = client.brand.retrieve(
    &BrandDto {
        url: "https://stripe.com".to_string(),
        refresh: None,
        timeout: None,
    },
    None,
).await?;
```

## Environments

This SDK allows you to configure different environments for API requests.

```rust
use berrycrawl::prelude::{*};

let config = ClientConfig {
    base_url: Environment::Production.url().to_string(),
    ..Default::default()
};
let client = Berrycrawl::new(config).expect("Failed to build client");
```

## Errors

When the API returns a non-success status code (4xx or 5xx response), an error will be returned.

```rust
match client.brand.retrieve(None)?.await {
    Ok(response) => {
        println!("Success: {:?}", response);
    },
    Err(ApiError::HTTP { status, message }) => {
        println!("API Error {}: {:?}", status, message);
    },
    Err(e) => {
        println!("Other error: {:?}", e);
    }
}
```

## Request Types

The SDK exports all request types as Rust structs. Simply import them from the crate to access them:

```rust
use berrycrawl::prelude::{*};

let request = BrandDto {
    ...
};
```

## Advanced

### Retries

The SDK is instrumented with automatic retries with exponential backoff. A request will be retried as long
as the request is deemed retryable and the number of retry attempts has not grown larger than the configured
retry limit (default: 2).

A request is deemed retryable when any of the following HTTP status codes is returned:

- [408](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/408) (Timeout)
- [429](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/429) (Too Many Requests)
- [5XX](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#server_error_responses) (Internal Server Error)

The `retryStatusCodes` configuration controls which [5XX](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#server_error_responses) status codes are retried:

- `legacy` (default): Retries `408`, `429`, and all `>= 500`
- `recommended`: Retries `408`, `429`, `502`, `503`, `504` only (excludes `500 Internal Server Error` to avoid retrying non-idempotent failures)

Use the `max_retries` method to configure this behavior.

```rust
let response = client.brand.retrieve(
    Some(RequestOptions::new().max_retries(3))
)?.await;
```

### Timeouts

The SDK defaults to a 30 second timeout. Use the `timeout` method to configure this behavior.

```rust
let response = client.brand.retrieve(
    Some(RequestOptions::new().timeout_seconds(30))
)?.await;
```

### Additional Headers

You can add custom headers to requests using `RequestOptions`.

```rust
let response = client.brand.retrieve(
    Some(
        RequestOptions::new()
            .additional_header("X-Custom-Header", "custom-value")
            .additional_header("X-Another-Header", "another-value")
    )
)?
.await;
```

### Additional Query String Parameters

You can add custom query parameters to requests using `RequestOptions`.

```rust
let response = client.brand.retrieve(
    Some(
        RequestOptions::new()
            .additional_query_param("filter", "active")
            .additional_query_param("sort", "desc")
    )
)?
.await;
```

## Contributing

While we value open-source contributions to this SDK, this library is generated programmatically.
Additions made directly to this library would have to be moved over to our generation code,
otherwise they would be overwritten upon the next generated release. Feel free to open a PR as
a proof of concept, but know that we will not be able to merge it as-is. We suggest opening
an issue first to discuss with us!

On the other hand, contributions to the README are always very welcome!
