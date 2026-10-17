pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BillingInterval {
    Monthly,
    Quarterly,
    SemiAnnual,
    Yearly,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BillingInterval {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Monthly => serializer.serialize_str("monthly"),
            Self::Quarterly => serializer.serialize_str("quarterly"),
            Self::SemiAnnual => serializer.serialize_str("semi_annual"),
            Self::Yearly => serializer.serialize_str("yearly"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BillingInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "monthly" => Ok(Self::Monthly),
            "quarterly" => Ok(Self::Quarterly),
            "semi_annual" => Ok(Self::SemiAnnual),
            "yearly" => Ok(Self::Yearly),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BillingInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Monthly => write!(f, "monthly"),
            Self::Quarterly => write!(f, "quarterly"),
            Self::SemiAnnual => write!(f, "semi_annual"),
            Self::Yearly => write!(f, "yearly"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
