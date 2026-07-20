pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChangeTrackingFormatDto {
    /// Change detection modes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<Vec<String>>,
    /// Schema for structured change tracking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<HashMap<String, serde_json::Value>>,
    /// Tag to identify this tracking session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Format type
    pub r#type: ChangeTrackingFormatDtoType,
}

impl ChangeTrackingFormatDto {
    pub fn builder() -> ChangeTrackingFormatDtoBuilder {
        <ChangeTrackingFormatDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChangeTrackingFormatDtoBuilder {
    modes: Option<Vec<String>>,
    schema: Option<HashMap<String, serde_json::Value>>,
    tag: Option<String>,
    r#type: Option<ChangeTrackingFormatDtoType>,
}

impl ChangeTrackingFormatDtoBuilder {
    pub fn modes(mut self, value: Vec<String>) -> Self {
        self.modes = Some(value);
        self
    }

    pub fn schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.schema = Some(value);
        self
    }

    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ChangeTrackingFormatDtoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChangeTrackingFormatDto`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ChangeTrackingFormatDtoBuilder::r#type)
    pub fn build(self) -> Result<ChangeTrackingFormatDto, BuildError> {
        Ok(ChangeTrackingFormatDto {
            modes: self.modes,
            schema: self.schema,
            tag: self.tag,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
