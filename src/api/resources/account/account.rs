use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AccountClient {
    pub http_client: HttpClient,
}

impl AccountClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn get(&self, options: Option<RequestOptions>) -> Result<AccountResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "account", None, None, options)
            .await
    }
}
