pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaxabilityReason {
    StandardRated,
    ReverseCharge,
    NotCollecting,
    CustomerExempt,
    ZeroRated,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaxabilityReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::StandardRated => serializer.serialize_str("standard_rated"),
            Self::ReverseCharge => serializer.serialize_str("reverse_charge"),
            Self::NotCollecting => serializer.serialize_str("not_collecting"),
            Self::CustomerExempt => serializer.serialize_str("customer_exempt"),
            Self::ZeroRated => serializer.serialize_str("zero_rated"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaxabilityReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "standard_rated" => Ok(Self::StandardRated),
            "reverse_charge" => Ok(Self::ReverseCharge),
            "not_collecting" => Ok(Self::NotCollecting),
            "customer_exempt" => Ok(Self::CustomerExempt),
            "zero_rated" => Ok(Self::ZeroRated),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaxabilityReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StandardRated => write!(f, "standard_rated"),
            Self::ReverseCharge => write!(f, "reverse_charge"),
            Self::NotCollecting => write!(f, "not_collecting"),
            Self::CustomerExempt => write!(f, "customer_exempt"),
            Self::ZeroRated => write!(f, "zero_rated"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
