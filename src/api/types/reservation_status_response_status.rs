pub use crate::prelude::*;

/// open, capturing, captured, released, expired, or recovery_pending.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReservationStatusResponseStatus {
    Open,
    Capturing,
    RecoveryPending,
    Captured,
    Released,
    Expired,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReservationStatusResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Open => serializer.serialize_str("open"),
            Self::Capturing => serializer.serialize_str("capturing"),
            Self::RecoveryPending => serializer.serialize_str("recovery_pending"),
            Self::Captured => serializer.serialize_str("captured"),
            Self::Released => serializer.serialize_str("released"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReservationStatusResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "open" => Ok(Self::Open),
            "capturing" => Ok(Self::Capturing),
            "recovery_pending" => Ok(Self::RecoveryPending),
            "captured" => Ok(Self::Captured),
            "released" => Ok(Self::Released),
            "expired" => Ok(Self::Expired),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReservationStatusResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Capturing => write!(f, "capturing"),
            Self::RecoveryPending => write!(f, "recovery_pending"),
            Self::Captured => write!(f, "captured"),
            Self::Released => write!(f, "released"),
            Self::Expired => write!(f, "expired"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
