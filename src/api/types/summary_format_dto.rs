pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SummaryFormatDto {
    /// Specific query for focused summarization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Format type
    pub r#type: SummaryFormatDtoType,
}

impl SummaryFormatDto {
    pub fn builder() -> SummaryFormatDtoBuilder {
        <SummaryFormatDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SummaryFormatDtoBuilder {
    query: Option<String>,
    r#type: Option<SummaryFormatDtoType>,
}

impl SummaryFormatDtoBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: SummaryFormatDtoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SummaryFormatDto`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](SummaryFormatDtoBuilder::r#type)
    pub fn build(self) -> Result<SummaryFormatDto, BuildError> {
        Ok(SummaryFormatDto {
            query: self.query,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
