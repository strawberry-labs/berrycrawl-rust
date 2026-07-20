pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ParseDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub timeout: Option<f64>,
    /// Public PDF, Word document, or spreadsheet URL
    #[serde(default)]
    pub url: String,
}

impl ParseDto {
    pub fn builder() -> ParseDtoBuilder {
        <ParseDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ParseDtoBuilder {
    timeout: Option<f64>,
    url: Option<String>,
}

impl ParseDtoBuilder {
    pub fn timeout(mut self, value: f64) -> Self {
        self.timeout = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ParseDto`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](ParseDtoBuilder::url)
    pub fn build(self) -> Result<ParseDto, BuildError> {
        Ok(ParseDto {
            timeout: self.timeout,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
