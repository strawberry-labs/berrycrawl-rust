pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScreenshotLocationDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
}

impl ScreenshotLocationDto {
    pub fn builder() -> ScreenshotLocationDtoBuilder {
        <ScreenshotLocationDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScreenshotLocationDtoBuilder {
    country: Option<String>,
    languages: Option<Vec<String>>,
}

impl ScreenshotLocationDtoBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn languages(mut self, value: Vec<String>) -> Self {
        self.languages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScreenshotLocationDto`].
    pub fn build(self) -> Result<ScreenshotLocationDto, BuildError> {
        Ok(ScreenshotLocationDto {
            country: self.country,
            languages: self.languages,
        })
    }
}
