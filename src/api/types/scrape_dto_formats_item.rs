pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ScrapeDtoFormatsItem {
    ScrapeDtoFormatsItemZero(ScrapeDtoFormatsItemZero),

    JsonFormatDto(JsonFormatDto),

    SummaryFormatDto(SummaryFormatDto),

    ChangeTrackingFormatDto(ChangeTrackingFormatDto),
}

impl ScrapeDtoFormatsItem {
    pub fn is_scrape_dto_formats_item_zero(&self) -> bool {
        matches!(self, Self::ScrapeDtoFormatsItemZero(_))
    }

    pub fn is_json_format_dto(&self) -> bool {
        matches!(self, Self::JsonFormatDto(_))
    }

    pub fn is_summary_format_dto(&self) -> bool {
        matches!(self, Self::SummaryFormatDto(_))
    }

    pub fn is_change_tracking_format_dto(&self) -> bool {
        matches!(self, Self::ChangeTrackingFormatDto(_))
    }

    pub fn as_scrape_dto_formats_item_zero(&self) -> Option<&ScrapeDtoFormatsItemZero> {
        match self {
            Self::ScrapeDtoFormatsItemZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_scrape_dto_formats_item_zero(self) -> Option<ScrapeDtoFormatsItemZero> {
        match self {
            Self::ScrapeDtoFormatsItemZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_json_format_dto(&self) -> Option<&JsonFormatDto> {
        match self {
            Self::JsonFormatDto(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_json_format_dto(self) -> Option<JsonFormatDto> {
        match self {
            Self::JsonFormatDto(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_summary_format_dto(&self) -> Option<&SummaryFormatDto> {
        match self {
            Self::SummaryFormatDto(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_summary_format_dto(self) -> Option<SummaryFormatDto> {
        match self {
            Self::SummaryFormatDto(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_change_tracking_format_dto(&self) -> Option<&ChangeTrackingFormatDto> {
        match self {
            Self::ChangeTrackingFormatDto(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_change_tracking_format_dto(self) -> Option<ChangeTrackingFormatDto> {
        match self {
            Self::ChangeTrackingFormatDto(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ScrapeDtoFormatsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ScrapeDtoFormatsItemZero(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::JsonFormatDto(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::SummaryFormatDto(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ChangeTrackingFormatDto(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
