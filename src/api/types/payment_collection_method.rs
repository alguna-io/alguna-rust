pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentCollectionMethod {
    Manual,
    Autopay,
    External,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentCollectionMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Manual => serializer.serialize_str("manual"),
            Self::Autopay => serializer.serialize_str("autopay"),
            Self::External => serializer.serialize_str("external"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "manual" => Ok(Self::Manual),
            "autopay" => Ok(Self::Autopay),
            "external" => Ok(Self::External),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentCollectionMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Manual => write!(f, "manual"),
            Self::Autopay => write!(f, "autopay"),
            Self::External => write!(f, "external"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
