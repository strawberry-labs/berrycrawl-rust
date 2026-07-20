pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ListJobsRequestType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListJobsRequestStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub page: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub limit: Option<f64>,
}

impl ListQueryRequest {
    pub fn builder() -> ListQueryRequestBuilder {
        <ListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListQueryRequestBuilder {
    r#type: Option<ListJobsRequestType>,
    status: Option<ListJobsRequestStatus>,
    page: Option<f64>,
    limit: Option<f64>,
}

impl ListQueryRequestBuilder {
    pub fn r#type(mut self, value: ListJobsRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn status(mut self, value: ListJobsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn page(mut self, value: f64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListQueryRequest`].
    pub fn build(self) -> Result<ListQueryRequest, BuildError> {
        Ok(ListQueryRequest {
            r#type: self.r#type,
            status: self.status,
            page: self.page,
            limit: self.limit,
        })
    }
}
