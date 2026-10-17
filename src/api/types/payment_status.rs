pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentStatus {
    Processing,
    Completed,
    Failed,
    PendingAuthorization,
    PendingClientAction,
    Voided,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Processing => serializer.serialize_str("processing"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::PendingAuthorization => serializer.serialize_str("pending_authorization"),
            Self::PendingClientAction => serializer.serialize_str("pending_client_action"),
            Self::Voided => serializer.serialize_str("voided"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "processing" => Ok(Self::Processing),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "pending_authorization" => Ok(Self::PendingAuthorization),
            "pending_client_action" => Ok(Self::PendingClientAction),
            "voided" => Ok(Self::Voided),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Processing => write!(f, "processing"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::PendingAuthorization => write!(f, "pending_authorization"),
            Self::PendingClientAction => write!(f, "pending_client_action"),
            Self::Voided => write!(f, "voided"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
