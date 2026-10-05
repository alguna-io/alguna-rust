pub use crate::prelude::*;

/// What to do when a settle cannot be fully covered by credit on a metric that stops at zero — either because it prices past the reservation's authorized hold, or because credit ran out before a late settle arrived. "reject" (the default) refuses the settle, burning and recording nothing. "record" burns what is available and reports the remainder in the charges' overflow, deliberately weakening a hard stop you configured. Reservation-settlement mode only. The refusal body's reason says which condition fired: adding credit resolves coverage_shortfall, but not envelope_exceeded, whose limit is the hold's size.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TrackCreditsRequestOnEnvelopeExceeded {
    Reject,
    Record,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TrackCreditsRequestOnEnvelopeExceeded {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Reject => serializer.serialize_str("reject"),
            Self::Record => serializer.serialize_str("record"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TrackCreditsRequestOnEnvelopeExceeded {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "reject" => Ok(Self::Reject),
            "record" => Ok(Self::Record),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TrackCreditsRequestOnEnvelopeExceeded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reject => write!(f, "reject"),
            Self::Record => write!(f, "record"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
