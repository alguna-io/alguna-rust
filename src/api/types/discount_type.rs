pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DiscountType {
    Percentage,
    Fixed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DiscountType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Percentage => serializer.serialize_str("percentage"),
            Self::Fixed => serializer.serialize_str("fixed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DiscountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "percentage" => Ok(Self::Percentage),
            "fixed" => Ok(Self::Fixed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DiscountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Percentage => write!(f, "percentage"),
            Self::Fixed => write!(f, "fixed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
