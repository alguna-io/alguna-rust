pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DurationType {
    Fixed,
    MonthlyRolling,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DurationType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Fixed => serializer.serialize_str("fixed"),
            Self::MonthlyRolling => serializer.serialize_str("monthly_rolling"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DurationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "fixed" => Ok(Self::Fixed),
            "monthly_rolling" => Ok(Self::MonthlyRolling),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DurationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixed => write!(f, "fixed"),
            Self::MonthlyRolling => write!(f, "monthly_rolling"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
