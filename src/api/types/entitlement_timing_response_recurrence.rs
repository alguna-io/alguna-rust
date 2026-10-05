pub use crate::prelude::*;

/// Recurrence type
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntitlementTimingResponseRecurrence {
    Recurring,
    OneOff,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EntitlementTimingResponseRecurrence {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Recurring => serializer.serialize_str("recurring"),
            Self::OneOff => serializer.serialize_str("one_off"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EntitlementTimingResponseRecurrence {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "recurring" => Ok(Self::Recurring),
            "one_off" => Ok(Self::OneOff),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EntitlementTimingResponseRecurrence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Recurring => write!(f, "recurring"),
            Self::OneOff => write!(f, "one_off"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
