pub use crate::prelude::*;

/// How the grant was created
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreditGrantResponseGrantType {
    Allocation,
    Manual,
    Rebalance,
    Api,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreditGrantResponseGrantType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Allocation => serializer.serialize_str("allocation"),
            Self::Manual => serializer.serialize_str("manual"),
            Self::Rebalance => serializer.serialize_str("rebalance"),
            Self::Api => serializer.serialize_str("api"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreditGrantResponseGrantType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "allocation" => Ok(Self::Allocation),
            "manual" => Ok(Self::Manual),
            "rebalance" => Ok(Self::Rebalance),
            "api" => Ok(Self::Api),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreditGrantResponseGrantType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allocation => write!(f, "allocation"),
            Self::Manual => write!(f, "manual"),
            Self::Rebalance => write!(f, "rebalance"),
            Self::Api => write!(f, "api"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
