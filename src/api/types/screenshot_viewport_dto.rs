pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScreenshotViewportDto {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub height: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub width: f64,
}

impl ScreenshotViewportDto {
    pub fn builder() -> ScreenshotViewportDtoBuilder {
        <ScreenshotViewportDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScreenshotViewportDtoBuilder {
    height: Option<f64>,
    width: Option<f64>,
}

impl ScreenshotViewportDtoBuilder {
    pub fn height(mut self, value: f64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn width(mut self, value: f64) -> Self {
        self.width = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScreenshotViewportDto`].
    /// This method will fail if any of the following fields are not set:
    /// - [`height`](ScreenshotViewportDtoBuilder::height)
    /// - [`width`](ScreenshotViewportDtoBuilder::width)
    pub fn build(self) -> Result<ScreenshotViewportDto, BuildError> {
        Ok(ScreenshotViewportDto {
            height: self
                .height
                .ok_or_else(|| BuildError::missing_field("height"))?,
            width: self
                .width
                .ok_or_else(|| BuildError::missing_field("width"))?,
        })
    }
}
