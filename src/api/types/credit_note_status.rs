pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreditNoteStatus {
    Draft,
    Applied,
    Void,
    PendingRefund,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreditNoteStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Draft => serializer.serialize_str("draft"),
            Self::Applied => serializer.serialize_str("applied"),
            Self::Void => serializer.serialize_str("void"),
            Self::PendingRefund => serializer.serialize_str("pending_refund"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreditNoteStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "draft" => Ok(Self::Draft),
            "applied" => Ok(Self::Applied),
            "void" => Ok(Self::Void),
            "pending_refund" => Ok(Self::PendingRefund),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreditNoteStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Applied => write!(f, "applied"),
            Self::Void => write!(f, "void"),
            Self::PendingRefund => write!(f, "pending_refund"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
