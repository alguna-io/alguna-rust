pub use crate::prelude::*;

/// Entitlement type
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntitlementResponseType {
    Boolean,
    Number,
    String_,
    CreditAllocation,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EntitlementResponseType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Boolean => serializer.serialize_str("boolean"),
            Self::Number => serializer.serialize_str("number"),
            Self::String_ => serializer.serialize_str("string"),
            Self::CreditAllocation => serializer.serialize_str("credit_allocation"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EntitlementResponseType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "boolean" => Ok(Self::Boolean),
            "number" => Ok(Self::Number),
            "string" => Ok(Self::String_),
            "credit_allocation" => Ok(Self::CreditAllocation),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EntitlementResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean => write!(f, "boolean"),
            Self::Number => write!(f, "number"),
            Self::String_ => write!(f, "string"),
            Self::CreditAllocation => write!(f, "credit_allocation"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
