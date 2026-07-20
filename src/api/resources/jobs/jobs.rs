use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct JobsClient {
    pub http_client: HttpClient,
}

impl JobsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list(
        &self,
        request: &ListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListJobsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "jobs",
                None,
                QueryBuilder::new()
                    .serialize("type", request.r#type.clone())
                    .serialize("status", request.status.clone())
                    .float("page", request.page.clone())
                    .float("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get(
        &self,
        id: &str,
        request: &GetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<JobResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("jobs/{}", id),
                None,
                QueryBuilder::new()
                    .float("page", request.page.clone())
                    .float("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn cancel(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CancelJobResponse, ApiError> {
        self.http_client
            .execute_request(Method::DELETE, &format!("jobs/{}", id), None, None, options)
            .await
    }
}
