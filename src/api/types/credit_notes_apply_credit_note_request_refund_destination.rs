pub use crate::prelude::*;

/// Where to send the refund when refunding a paid invoice
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApplyCreditNoteRequestRefundDestination {
    OriginalPayment,
    Wallet,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ApplyCreditNoteRequestRefundDestination {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OriginalPayment => serializer.serialize_str("original_payment"),
            Self::Wallet => serializer.serialize_str("wallet"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ApplyCreditNoteRequestRefundDestination {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "original_payment" => Ok(Self::OriginalPayment),
            "wallet" => Ok(Self::Wallet),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ApplyCreditNoteRequestRefundDestination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OriginalPayment => write!(f, "original_payment"),
            Self::Wallet => write!(f, "wallet"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
