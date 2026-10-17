pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaggableEntityType {
    Product,
    Customer,
    Plan,
    Subscription,
    Metric,
    Invoice,
    Quote,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaggableEntityType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Product => serializer.serialize_str("product"),
            Self::Customer => serializer.serialize_str("customer"),
            Self::Plan => serializer.serialize_str("plan"),
            Self::Subscription => serializer.serialize_str("subscription"),
            Self::Metric => serializer.serialize_str("metric"),
            Self::Invoice => serializer.serialize_str("invoice"),
            Self::Quote => serializer.serialize_str("quote"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaggableEntityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "product" => Ok(Self::Product),
            "customer" => Ok(Self::Customer),
            "plan" => Ok(Self::Plan),
            "subscription" => Ok(Self::Subscription),
            "metric" => Ok(Self::Metric),
            "invoice" => Ok(Self::Invoice),
            "quote" => Ok(Self::Quote),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaggableEntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Product => write!(f, "product"),
            Self::Customer => write!(f, "customer"),
            Self::Plan => write!(f, "plan"),
            Self::Subscription => write!(f, "subscription"),
            Self::Metric => write!(f, "metric"),
            Self::Invoice => write!(f, "invoice"),
            Self::Quote => write!(f, "quote"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
