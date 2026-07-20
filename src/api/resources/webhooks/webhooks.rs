use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct WebhooksClient {
    pub http_client: HttpClient,
}

impl WebhooksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<ListWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "webhooks", None, None, options)
            .await
    }

    pub async fn create(
        &self,
        request: &CreateWebhookDto,
        options: Option<RequestOptions>,
    ) -> Result<WebhookResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "webhooks",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WebhookResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("webhooks/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("webhooks/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn update(
        &self,
        id: &str,
        request: &UpdateWebhookDto,
        options: Option<RequestOptions>,
    ) -> Result<WebhookResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("webhooks/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn test(
        &self,
        id: &str,
        request: &TestWebhookDto,
        options: Option<RequestOptions>,
    ) -> Result<TestWebhookResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("webhooks/{}/test", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
