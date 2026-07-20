use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct BrandClient {
    pub http_client: HttpClient,
}

impl BrandClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Send one website URL. BerryCrawl returns the brand identity found on that site.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn retrieve(
        &self,
        request: &BrandDto,
        options: Option<RequestOptions>,
    ) -> Result<BrandResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "brand",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
