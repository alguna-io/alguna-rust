pub use crate::prelude::*;

/// Initial invoice status. Defaults to draft. Setting issued releases the invoice for issuance, which completes asynchronously: the response returns the invoice in processing, and it settles to issued shortly after — poll the Get Invoice endpoint to observe the final status. paid is only valid on the tax_calculation_id path and must be sent together with a payment.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateInvoiceRequestStatus {
    Draft,
    Issued,
    Paid,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateInvoiceRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Draft => serializer.serialize_str("draft"),
            Self::Issued => serializer.serialize_str("issued"),
            Self::Paid => serializer.serialize_str("paid"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateInvoiceRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "draft" => Ok(Self::Draft),
            "issued" => Ok(Self::Issued),
            "paid" => Ok(Self::Paid),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateInvoiceRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Issued => write!(f, "issued"),
            Self::Paid => write!(f, "paid"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
