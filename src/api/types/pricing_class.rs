pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PricingClass {
    Fixed,
    Metered,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PricingClass {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Fixed => serializer.serialize_str("fixed"),
            Self::Metered => serializer.serialize_str("metered"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PricingClass {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "fixed" => Ok(Self::Fixed),
            "metered" => Ok(Self::Metered),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PricingClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixed => write!(f, "fixed"),
            Self::Metered => write!(f, "metered"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
