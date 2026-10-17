pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaxType {
    Gst,
    Hst,
    Igst,
    Jct,
    LeaseTax,
    Pst,
    Qst,
    SalesTax,
    ServiceTax,
    Vat,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaxType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Gst => serializer.serialize_str("gst"),
            Self::Hst => serializer.serialize_str("hst"),
            Self::Igst => serializer.serialize_str("igst"),
            Self::Jct => serializer.serialize_str("jct"),
            Self::LeaseTax => serializer.serialize_str("lease_tax"),
            Self::Pst => serializer.serialize_str("pst"),
            Self::Qst => serializer.serialize_str("qst"),
            Self::SalesTax => serializer.serialize_str("sales_tax"),
            Self::ServiceTax => serializer.serialize_str("service_tax"),
            Self::Vat => serializer.serialize_str("vat"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaxType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "gst" => Ok(Self::Gst),
            "hst" => Ok(Self::Hst),
            "igst" => Ok(Self::Igst),
            "jct" => Ok(Self::Jct),
            "lease_tax" => Ok(Self::LeaseTax),
            "pst" => Ok(Self::Pst),
            "qst" => Ok(Self::Qst),
            "sales_tax" => Ok(Self::SalesTax),
            "service_tax" => Ok(Self::ServiceTax),
            "vat" => Ok(Self::Vat),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gst => write!(f, "gst"),
            Self::Hst => write!(f, "hst"),
            Self::Igst => write!(f, "igst"),
            Self::Jct => write!(f, "jct"),
            Self::LeaseTax => write!(f, "lease_tax"),
            Self::Pst => write!(f, "pst"),
            Self::Qst => write!(f, "qst"),
            Self::SalesTax => write!(f, "sales_tax"),
            Self::ServiceTax => write!(f, "service_tax"),
            Self::Vat => write!(f, "vat"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
