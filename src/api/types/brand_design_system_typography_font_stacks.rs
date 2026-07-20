pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrandDesignSystemTypographyFontStacks {
    #[serde(default)]
    pub body: Vec<String>,
    #[serde(default)]
    pub heading: Vec<String>,
    #[serde(default)]
    pub paragraph: Vec<String>,
}

impl BrandDesignSystemTypographyFontStacks {
    pub fn builder() -> BrandDesignSystemTypographyFontStacksBuilder {
        <BrandDesignSystemTypographyFontStacksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandDesignSystemTypographyFontStacksBuilder {
    body: Option<Vec<String>>,
    heading: Option<Vec<String>>,
    paragraph: Option<Vec<String>>,
}

impl BrandDesignSystemTypographyFontStacksBuilder {
    pub fn body(mut self, value: Vec<String>) -> Self {
        self.body = Some(value);
        self
    }

    pub fn heading(mut self, value: Vec<String>) -> Self {
        self.heading = Some(value);
        self
    }

    pub fn paragraph(mut self, value: Vec<String>) -> Self {
        self.paragraph = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BrandDesignSystemTypographyFontStacks`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](BrandDesignSystemTypographyFontStacksBuilder::body)
    /// - [`heading`](BrandDesignSystemTypographyFontStacksBuilder::heading)
    /// - [`paragraph`](BrandDesignSystemTypographyFontStacksBuilder::paragraph)
    pub fn build(self) -> Result<BrandDesignSystemTypographyFontStacks, BuildError> {
        Ok(BrandDesignSystemTypographyFontStacks {
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            heading: self
                .heading
                .ok_or_else(|| BuildError::missing_field("heading"))?,
            paragraph: self
                .paragraph
                .ok_or_else(|| BuildError::missing_field("paragraph"))?,
        })
    }
}
