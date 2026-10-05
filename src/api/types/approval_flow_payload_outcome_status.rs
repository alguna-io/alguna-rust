pub use crate::prelude::*;

/// Status of the operation the flow gates
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApprovalFlowPayloadOutcomeStatus {
    None,
    Pending,
    ApprovedAwaitingExecution,
    ApprovedManualHold,
    Executed,
    Stale,
    Failed,
    Rejected,
    Canceled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ApprovalFlowPayloadOutcomeStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::ApprovedAwaitingExecution => {
                serializer.serialize_str("approved_awaiting_execution")
            }
            Self::ApprovedManualHold => serializer.serialize_str("approved_manual_hold"),
            Self::Executed => serializer.serialize_str("executed"),
            Self::Stale => serializer.serialize_str("stale"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ApprovalFlowPayloadOutcomeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "none" => Ok(Self::None),
            "pending" => Ok(Self::Pending),
            "approved_awaiting_execution" => Ok(Self::ApprovedAwaitingExecution),
            "approved_manual_hold" => Ok(Self::ApprovedManualHold),
            "executed" => Ok(Self::Executed),
            "stale" => Ok(Self::Stale),
            "failed" => Ok(Self::Failed),
            "rejected" => Ok(Self::Rejected),
            "canceled" => Ok(Self::Canceled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ApprovalFlowPayloadOutcomeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Pending => write!(f, "pending"),
            Self::ApprovedAwaitingExecution => write!(f, "approved_awaiting_execution"),
            Self::ApprovedManualHold => write!(f, "approved_manual_hold"),
            Self::Executed => write!(f, "executed"),
            Self::Stale => write!(f, "stale"),
            Self::Failed => write!(f, "failed"),
            Self::Rejected => write!(f, "rejected"),
            Self::Canceled => write!(f, "canceled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
