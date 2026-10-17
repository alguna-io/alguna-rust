pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentProvider {
    Stripe,
    Orum,
    Forward,
    Primer,
    OfflinePayment,
    AlgunaWallet,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentProvider {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Stripe => serializer.serialize_str("stripe"),
            Self::Orum => serializer.serialize_str("orum"),
            Self::Forward => serializer.serialize_str("forward"),
            Self::Primer => serializer.serialize_str("primer"),
            Self::OfflinePayment => serializer.serialize_str("offline_payment"),
            Self::AlgunaWallet => serializer.serialize_str("alguna_wallet"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "stripe" => Ok(Self::Stripe),
            "orum" => Ok(Self::Orum),
            "forward" => Ok(Self::Forward),
            "primer" => Ok(Self::Primer),
            "offline_payment" => Ok(Self::OfflinePayment),
            "alguna_wallet" => Ok(Self::AlgunaWallet),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stripe => write!(f, "stripe"),
            Self::Orum => write!(f, "orum"),
            Self::Forward => write!(f, "forward"),
            Self::Primer => write!(f, "primer"),
            Self::OfflinePayment => write!(f, "offline_payment"),
            Self::AlgunaWallet => write!(f, "alguna_wallet"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
