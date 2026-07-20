pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Webhook {
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(rename = "recentDeliveries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_deliveries: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub url: String,
}

impl Webhook {
    pub fn builder() -> WebhookBuilder {
        <WebhookBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    events: Option<Vec<String>>,
    id: Option<String>,
    is_active: Option<bool>,
    recent_deliveries: Option<Vec<HashMap<String, serde_json::Value>>>,
    secret: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    url: Option<String>,
}

impl WebhookBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<String>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn recent_deliveries(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.recent_deliveries = Some(value);
        self
    }

    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Webhook`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](WebhookBuilder::events)
    /// - [`id`](WebhookBuilder::id)
    /// - [`is_active`](WebhookBuilder::is_active)
    /// - [`url`](WebhookBuilder::url)
    pub fn build(self) -> Result<Webhook, BuildError> {
        Ok(Webhook {
            created_at: self.created_at,
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            recent_deliveries: self.recent_deliveries,
            secret: self.secret,
            updated_at: self.updated_at,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
