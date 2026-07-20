pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BrandDesignSystemSpacing {
    #[serde(rename = "baseUnit")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub base_unit: f64,
    #[serde(rename = "borderRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<String>,
}

impl BrandDesignSystemSpacing {
    pub fn builder() -> BrandDesignSystemSpacingBuilder {
        <BrandDesignSystemSpacingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandDesignSystemSpacingBuilder {
    base_unit: Option<f64>,
    border_radius: Option<String>,
}

impl BrandDesignSystemSpacingBuilder {
    pub fn base_unit(mut self, value: f64) -> Self {
        self.base_unit = Some(value);
        self
    }

    pub fn border_radius(mut self, value: impl Into<String>) -> Self {
        self.border_radius = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrandDesignSystemSpacing`].
    /// This method will fail if any of the following fields are not set:
    /// - [`base_unit`](BrandDesignSystemSpacingBuilder::base_unit)
    pub fn build(self) -> Result<BrandDesignSystemSpacing, BuildError> {
        Ok(BrandDesignSystemSpacing {
            base_unit: self
                .base_unit
                .ok_or_else(|| BuildError::missing_field("base_unit"))?,
            border_radius: self.border_radius,
        })
    }
}
