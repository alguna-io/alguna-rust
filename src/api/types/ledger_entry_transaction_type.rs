pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LedgerEntryTransactionType {
    CreditGrant,
    Payment,
    Refund,
    ManualDebit,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LedgerEntryTransactionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreditGrant => serializer.serialize_str("credit_grant"),
            Self::Payment => serializer.serialize_str("payment"),
            Self::Refund => serializer.serialize_str("refund"),
            Self::ManualDebit => serializer.serialize_str("manual_debit"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LedgerEntryTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "credit_grant" => Ok(Self::CreditGrant),
            "payment" => Ok(Self::Payment),
            "refund" => Ok(Self::Refund),
            "manual_debit" => Ok(Self::ManualDebit),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LedgerEntryTransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreditGrant => write!(f, "credit_grant"),
            Self::Payment => write!(f, "payment"),
            Self::Refund => write!(f, "refund"),
            Self::ManualDebit => write!(f, "manual_debit"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
