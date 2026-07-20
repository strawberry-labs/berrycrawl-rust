pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScreenshotCookieDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "httpOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_only: Option<bool>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "sameSite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_site: Option<ScreenshotCookieDtoSameSite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    #[serde(default)]
    pub value: String,
}

impl ScreenshotCookieDto {
    pub fn builder() -> ScreenshotCookieDtoBuilder {
        <ScreenshotCookieDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScreenshotCookieDtoBuilder {
    domain: Option<String>,
    http_only: Option<bool>,
    name: Option<String>,
    path: Option<String>,
    same_site: Option<ScreenshotCookieDtoSameSite>,
    secure: Option<bool>,
    value: Option<String>,
}

impl ScreenshotCookieDtoBuilder {
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn http_only(mut self, value: bool) -> Self {
        self.http_only = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn same_site(mut self, value: ScreenshotCookieDtoSameSite) -> Self {
        self.same_site = Some(value);
        self
    }

    pub fn secure(mut self, value: bool) -> Self {
        self.secure = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScreenshotCookieDto`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ScreenshotCookieDtoBuilder::name)
    /// - [`value`](ScreenshotCookieDtoBuilder::value)
    pub fn build(self) -> Result<ScreenshotCookieDto, BuildError> {
        Ok(ScreenshotCookieDto {
            domain: self.domain,
            http_only: self.http_only,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path,
            same_site: self.same_site,
            secure: self.secure,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
