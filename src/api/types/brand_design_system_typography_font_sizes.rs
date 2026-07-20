pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrandDesignSystemTypographyFontSizes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h2: Option<String>,
}

impl BrandDesignSystemTypographyFontSizes {
    pub fn builder() -> BrandDesignSystemTypographyFontSizesBuilder {
        <BrandDesignSystemTypographyFontSizesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandDesignSystemTypographyFontSizesBuilder {
    body: Option<String>,
    h1: Option<String>,
    h2: Option<String>,
}

impl BrandDesignSystemTypographyFontSizesBuilder {
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn h1(mut self, value: impl Into<String>) -> Self {
        self.h1 = Some(value.into());
        self
    }

    pub fn h2(mut self, value: impl Into<String>) -> Self {
        self.h2 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrandDesignSystemTypographyFontSizes`].
    pub fn build(self) -> Result<BrandDesignSystemTypographyFontSizes, BuildError> {
        Ok(BrandDesignSystemTypographyFontSizes {
            body: self.body,
            h1: self.h1,
            h2: self.h2,
        })
    }
}
