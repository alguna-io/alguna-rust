pub use crate::prelude::*;

/// Time resolution of data points
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AmountsOverTimeResponseResolution {
    Months,
    Days,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AmountsOverTimeResponseResolution {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Months => serializer.serialize_str("months"),
            Self::Days => serializer.serialize_str("days"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AmountsOverTimeResponseResolution {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "months" => Ok(Self::Months),
            "days" => Ok(Self::Days),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AmountsOverTimeResponseResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Months => write!(f, "months"),
            Self::Days => write!(f, "days"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
