pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExternalPaymentSource {
    BankTransfer,
    Cheque,
    PaymentCard,
    Cash,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExternalPaymentSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::BankTransfer => serializer.serialize_str("bank_transfer"),
            Self::Cheque => serializer.serialize_str("cheque"),
            Self::PaymentCard => serializer.serialize_str("payment_card"),
            Self::Cash => serializer.serialize_str("cash"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExternalPaymentSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "bank_transfer" => Ok(Self::BankTransfer),
            "cheque" => Ok(Self::Cheque),
            "payment_card" => Ok(Self::PaymentCard),
            "cash" => Ok(Self::Cash),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExternalPaymentSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BankTransfer => write!(f, "bank_transfer"),
            Self::Cheque => write!(f, "cheque"),
            Self::PaymentCard => write!(f, "payment_card"),
            Self::Cash => write!(f, "cash"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
