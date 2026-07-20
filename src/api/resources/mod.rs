//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Account**
//! - **Brand**
//! - **Jobs**
//! - **Webhooks**

use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod account;
pub mod brand;
pub mod jobs;
pub mod webhooks;
pub struct Berrycrawl {
    pub config: ClientConfig,
    pub http_client: HttpClient,
    pub account: AccountClient,
    pub brand: BrandClient,
    pub jobs: JobsClient,
    pub webhooks: WebhooksClient,
}

impl Berrycrawl {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            http_client: HttpClient::new(config.clone())?,
            account: AccountClient::new(config.clone())?,
            brand: BrandClient::new(config.clone())?,
            jobs: JobsClient::new(config.clone())?,
            webhooks: WebhooksClient::new(config.clone())?,
        })
    }

    pub async fn crawl(
        &self,
        request: &CrawlDto,
        options: Option<RequestOptions>,
    ) -> Result<JobCreatedResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "crawl",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn extract(
        &self,
        request: &ExtractDto,
        options: Option<RequestOptions>,
    ) -> Result<JobCreatedResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "extract",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn map(
        &self,
        request: &MapDto,
        options: Option<RequestOptions>,
    ) -> Result<MapResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "map",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn parse(
        &self,
        request: &ParseDto,
        options: Option<RequestOptions>,
    ) -> Result<ScrapeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "parse",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn scrape(
        &self,
        request: &ScrapeDto,
        options: Option<RequestOptions>,
    ) -> Result<ScrapeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "scrape",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cookie banners, blocking overlays, chat widgets, and lazy loading are handled by default. Every cleanup pass can be controlled per request.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn screenshot(
        &self,
        request: &ScreenshotDto,
        options: Option<RequestOptions>,
    ) -> Result<ScrapeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "screenshot",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn search(
        &self,
        request: &SearchDto,
        options: Option<RequestOptions>,
    ) -> Result<SearchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "search",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}

pub use account::AccountClient;
pub use brand::BrandClient;
pub use jobs::JobsClient;
pub use webhooks::WebhooksClient;
