pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ActionDtoDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub milliseconds: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub r#type: ActionDtoType,
}

impl ActionDto {
    pub fn builder() -> ActionDtoBuilder {
        <ActionDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActionDtoBuilder {
    amount: Option<f64>,
    direction: Option<ActionDtoDirection>,
    key: Option<String>,
    milliseconds: Option<f64>,
    selector: Option<String>,
    text: Option<String>,
    r#type: Option<ActionDtoType>,
}

impl ActionDtoBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn direction(mut self, value: ActionDtoDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn milliseconds(mut self, value: f64) -> Self {
        self.milliseconds = Some(value);
        self
    }

    pub fn selector(mut self, value: impl Into<String>) -> Self {
        self.selector = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ActionDtoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ActionDto`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ActionDtoBuilder::r#type)
    pub fn build(self) -> Result<ActionDto, BuildError> {
        Ok(ActionDto {
            amount: self.amount,
            direction: self.direction,
            key: self.key,
            milliseconds: self.milliseconds,
            selector: self.selector,
            text: self.text,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
