pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrandComponentStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(rename = "borderColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
    #[serde(rename = "borderRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<String>,
    #[serde(rename = "textColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
}

impl BrandComponentStyle {
    pub fn builder() -> BrandComponentStyleBuilder {
        <BrandComponentStyleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandComponentStyleBuilder {
    background: Option<String>,
    border_color: Option<String>,
    border_radius: Option<String>,
    shadow: Option<String>,
    text_color: Option<String>,
}

impl BrandComponentStyleBuilder {
    pub fn background(mut self, value: impl Into<String>) -> Self {
        self.background = Some(value.into());
        self
    }

    pub fn border_color(mut self, value: impl Into<String>) -> Self {
        self.border_color = Some(value.into());
        self
    }

    pub fn border_radius(mut self, value: impl Into<String>) -> Self {
        self.border_radius = Some(value.into());
        self
    }

    pub fn shadow(mut self, value: impl Into<String>) -> Self {
        self.shadow = Some(value.into());
        self
    }

    pub fn text_color(mut self, value: impl Into<String>) -> Self {
        self.text_color = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrandComponentStyle`].
    pub fn build(self) -> Result<BrandComponentStyle, BuildError> {
        Ok(BrandComponentStyle {
            background: self.background,
            border_color: self.border_color,
            border_radius: self.border_radius,
            shadow: self.shadow,
            text_color: self.text_color,
        })
    }
}
