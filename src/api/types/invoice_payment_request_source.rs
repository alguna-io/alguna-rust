pub use crate::prelude::*;

/// How the customer paid
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InvoicePaymentRequestSource {
    BankTransfer,
    PaymentCard,
    Cash,
    Cheque,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InvoicePaymentRequestSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::BankTransfer => serializer.serialize_str("bank_transfer"),
            Self::PaymentCard => serializer.serialize_str("payment_card"),
            Self::Cash => serializer.serialize_str("cash"),
            Self::Cheque => serializer.serialize_str("cheque"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InvoicePaymentRequestSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "bank_transfer" => Ok(Self::BankTransfer),
            "payment_card" => Ok(Self::PaymentCard),
            "cash" => Ok(Self::Cash),
            "cheque" => Ok(Self::Cheque),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InvoicePaymentRequestSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BankTransfer => write!(f, "bank_transfer"),
            Self::PaymentCard => write!(f, "payment_card"),
            Self::Cash => write!(f, "cash"),
            Self::Cheque => write!(f, "cheque"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
