pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InvoiceStatus {
    Draft,
    Upcoming,
    PendingApproval,
    Scheduled,
    Processing,
    Issued,
    Void,
    Paid,
    Canceled,
    Overdue,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InvoiceStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Draft => serializer.serialize_str("draft"),
            Self::Upcoming => serializer.serialize_str("upcoming"),
            Self::PendingApproval => serializer.serialize_str("pending_approval"),
            Self::Scheduled => serializer.serialize_str("scheduled"),
            Self::Processing => serializer.serialize_str("processing"),
            Self::Issued => serializer.serialize_str("issued"),
            Self::Void => serializer.serialize_str("void"),
            Self::Paid => serializer.serialize_str("paid"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::Overdue => serializer.serialize_str("overdue"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InvoiceStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "draft" => Ok(Self::Draft),
            "upcoming" => Ok(Self::Upcoming),
            "pending_approval" => Ok(Self::PendingApproval),
            "scheduled" => Ok(Self::Scheduled),
            "processing" => Ok(Self::Processing),
            "issued" => Ok(Self::Issued),
            "void" => Ok(Self::Void),
            "paid" => Ok(Self::Paid),
            "canceled" => Ok(Self::Canceled),
            "overdue" => Ok(Self::Overdue),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Upcoming => write!(f, "upcoming"),
            Self::PendingApproval => write!(f, "pending_approval"),
            Self::Scheduled => write!(f, "scheduled"),
            Self::Processing => write!(f, "processing"),
            Self::Issued => write!(f, "issued"),
            Self::Void => write!(f, "void"),
            Self::Paid => write!(f, "paid"),
            Self::Canceled => write!(f, "canceled"),
            Self::Overdue => write!(f, "overdue"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
