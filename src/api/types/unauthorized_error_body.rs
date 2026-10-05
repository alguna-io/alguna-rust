pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum UnauthorizedErrorBody {
    ErrorResponse(ErrorResponse),

    GatewayErrorResponse(GatewayErrorResponse),
}

impl UnauthorizedErrorBody {
    pub fn is_error_response(&self) -> bool {
        matches!(self, Self::ErrorResponse(_))
    }

    pub fn is_gateway_error_response(&self) -> bool {
        matches!(self, Self::GatewayErrorResponse(_))
    }

    pub fn as_error_response(&self) -> Option<&ErrorResponse> {
        match self {
            Self::ErrorResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_error_response(self) -> Option<ErrorResponse> {
        match self {
            Self::ErrorResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_gateway_error_response(&self) -> Option<&GatewayErrorResponse> {
        match self {
            Self::GatewayErrorResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_gateway_error_response(self) -> Option<GatewayErrorResponse> {
        match self {
            Self::GatewayErrorResponse(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for UnauthorizedErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ErrorResponse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::GatewayErrorResponse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
