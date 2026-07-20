pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonFormatDto {
    /// Natural language prompt for extraction guidance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// JSON schema for structured extraction
    #[serde(default)]
    pub schema: HashMap<String, serde_json::Value>,
    /// Format type
    pub r#type: JsonFormatDtoType,
}

impl JsonFormatDto {
    pub fn builder() -> JsonFormatDtoBuilder {
        <JsonFormatDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JsonFormatDtoBuilder {
    prompt: Option<String>,
    schema: Option<HashMap<String, serde_json::Value>>,
    r#type: Option<JsonFormatDtoType>,
}

impl JsonFormatDtoBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.schema = Some(value);
        self
    }

    pub fn r#type(mut self, value: JsonFormatDtoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JsonFormatDto`].
    /// This method will fail if any of the following fields are not set:
    /// - [`schema`](JsonFormatDtoBuilder::schema)
    /// - [`r#type`](JsonFormatDtoBuilder::r#type)
    pub fn build(self) -> Result<JsonFormatDto, BuildError> {
        Ok(JsonFormatDto {
            prompt: self.prompt,
            schema: self
                .schema
                .ok_or_else(|| BuildError::missing_field("schema"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
