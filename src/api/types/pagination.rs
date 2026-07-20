pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Pagination {
    #[serde(default)]
    pub limit: i64,
    #[serde(default)]
    pub page: i64,
    #[serde(default)]
    pub total: i64,
    #[serde(rename = "totalPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
}

impl Pagination {
    pub fn builder() -> PaginationBuilder {
        <PaginationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginationBuilder {
    limit: Option<i64>,
    page: Option<i64>,
    total: Option<i64>,
    total_pages: Option<i64>,
}

impl PaginationBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn total_pages(mut self, value: i64) -> Self {
        self.total_pages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Pagination`].
    /// This method will fail if any of the following fields are not set:
    /// - [`limit`](PaginationBuilder::limit)
    /// - [`page`](PaginationBuilder::page)
    /// - [`total`](PaginationBuilder::total)
    pub fn build(self) -> Result<Pagination, BuildError> {
        Ok(Pagination {
            limit: self
                .limit
                .ok_or_else(|| BuildError::missing_field("limit"))?,
            page: self.page.ok_or_else(|| BuildError::missing_field("page"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            total_pages: self.total_pages,
        })
    }
}
