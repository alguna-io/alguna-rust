pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubscriptionCancelationReason {
    NoLongerRequired,
    MovingProvider,
    Pricing,
    Support,
    Features,
    Upgraded,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SubscriptionCancelationReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NoLongerRequired => serializer.serialize_str("no_longer_required"),
            Self::MovingProvider => serializer.serialize_str("moving_provider"),
            Self::Pricing => serializer.serialize_str("pricing"),
            Self::Support => serializer.serialize_str("support"),
            Self::Features => serializer.serialize_str("features"),
            Self::Upgraded => serializer.serialize_str("upgraded"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SubscriptionCancelationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "no_longer_required" => Ok(Self::NoLongerRequired),
            "moving_provider" => Ok(Self::MovingProvider),
            "pricing" => Ok(Self::Pricing),
            "support" => Ok(Self::Support),
            "features" => Ok(Self::Features),
            "upgraded" => Ok(Self::Upgraded),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SubscriptionCancelationReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoLongerRequired => write!(f, "no_longer_required"),
            Self::MovingProvider => write!(f, "moving_provider"),
            Self::Pricing => write!(f, "pricing"),
            Self::Support => write!(f, "support"),
            Self::Features => write!(f, "features"),
            Self::Upgraded => write!(f, "upgraded"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
