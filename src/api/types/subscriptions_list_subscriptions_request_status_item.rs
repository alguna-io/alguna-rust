pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListSubscriptionsRequestStatusItem {
    Draft,
    Scheduled,
    Active,
    Paused,
    Canceled,
    Completed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListSubscriptionsRequestStatusItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Draft => serializer.serialize_str("draft"),
            Self::Scheduled => serializer.serialize_str("scheduled"),
            Self::Active => serializer.serialize_str("active"),
            Self::Paused => serializer.serialize_str("paused"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListSubscriptionsRequestStatusItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "draft" => Ok(Self::Draft),
            "scheduled" => Ok(Self::Scheduled),
            "active" => Ok(Self::Active),
            "paused" => Ok(Self::Paused),
            "canceled" => Ok(Self::Canceled),
            "completed" => Ok(Self::Completed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListSubscriptionsRequestStatusItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Scheduled => write!(f, "scheduled"),
            Self::Active => write!(f, "active"),
            Self::Paused => write!(f, "paused"),
            Self::Canceled => write!(f, "canceled"),
            Self::Completed => write!(f, "completed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
