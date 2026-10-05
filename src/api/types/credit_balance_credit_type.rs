pub use crate::prelude::*;

/// The credit denomination of this entry.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreditBalanceCreditType {
    Monetary,
    Units,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreditBalanceCreditType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Monetary => serializer.serialize_str("monetary"),
            Self::Units => serializer.serialize_str("units"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreditBalanceCreditType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "monetary" => Ok(Self::Monetary),
            "units" => Ok(Self::Units),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreditBalanceCreditType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Monetary => write!(f, "monetary"),
            Self::Units => write!(f, "units"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
