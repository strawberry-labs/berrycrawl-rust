pub use crate::prelude::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub page: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub limit: Option<f64>,
}

impl GetQueryRequest {
    pub fn builder() -> GetQueryRequestBuilder {
        <GetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetQueryRequestBuilder {
    page: Option<f64>,
    limit: Option<f64>,
}

impl GetQueryRequestBuilder {
    pub fn page(mut self, value: f64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetQueryRequest`].
    pub fn build(self) -> Result<GetQueryRequest, BuildError> {
        Ok(GetQueryRequest {
            page: self.page,
            limit: self.limit,
        })
    }
}
