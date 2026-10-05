use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ApiError {
    #[error("BadRequestError: Bad request - {message}")]
    BadRequestError {
        message: String,
        detail: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        type_: Option<String>,
    },
    #[error("UnauthorizedError: Authentication failed - {message}")]
    UnauthorizedError {
        message: String,
        auth_type: Option<String>,
    },
    #[error("ForbiddenError: Access forbidden - {message}")]
    ForbiddenError {
        message: String,
        detail: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        type_: Option<String>,
    },
    #[error("NotFoundError: Resource not found - {message}")]
    NotFoundError {
        message: String,
        detail: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        type_: Option<String>,
    },
    #[error("UnprocessableEntityError: Unprocessable entity - {message}")]
    UnprocessableEntityError {
        message: String,
        detail: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        type_: Option<String>,
    },
    #[error("InternalServerError: Internal server error - {message}")]
    InternalServerError {
        message: String,
        detail: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        type_: Option<String>,
    },
    #[error("BadGatewayError: {message}")]
    BadGatewayError { message: String },
    #[error("ServiceUnavailableError: {message}")]
    ServiceUnavailableError {
        message: String,
        detail: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        type_: Option<String>,
    },
    #[error("GatewayTimeoutError: {message}")]
    GatewayTimeoutError {
        message: String,
        detail: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        type_: Option<String>,
    },
    #[error("ConflictError: Conflict - {message}")]
    ConflictError {
        message: String,
        detail: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        type_: Option<String>,
    },
    #[error("TooManyRequestsError: Rate limit exceeded - {message}")]
    TooManyRequestsError {
        message: String,
        detail: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        type_: Option<String>,
    },
    #[error("HTTP error {status}: {message}")]
    Http { status: u16, message: String },
    #[error("Network error: {0}")]
    Network(#[source] reqwest::Error),
    #[error("Request executor error: {0}")]
    Executor(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Serialization error: {0}")]
    Serialization(#[source] serde_json::Error),
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Invalid header value")]
    InvalidHeader,
    #[error("Could not clone request for retry")]
    RequestClone,
    #[error("SSE stream terminated")]
    StreamTerminated,
    #[error("SSE stream timed out waiting for next event")]
    StreamTimeout,
    #[error("SSE parse error: {0}")]
    SseParseError(String),
}

impl ApiError {
    pub fn from_response(status_code: u16, body: Option<&str>) -> Self {
        match status_code {
            400 => {
                // Parse error body for BadRequestError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::BadRequestError {
                            message: crate::ext::error_message(status_code, Some(&parsed)),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::BadRequestError {
                    message: crate::ext::error_message(status_code, None),
                    detail: None,
                    status: None,
                    title: None,
                    type_: None,
                };
            }
            401 => {
                // Parse error body for UnauthorizedError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::UnauthorizedError {
                            message: crate::ext::error_message(status_code, Some(&parsed)),
                            auth_type: parsed
                                .get("authType")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::UnauthorizedError {
                    message: crate::ext::error_message(status_code, None),
                    auth_type: None,
                };
            }
            403 => {
                // Parse error body for ForbiddenError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ForbiddenError {
                            message: crate::ext::error_message(status_code, Some(&parsed)),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::ForbiddenError {
                    message: crate::ext::error_message(status_code, None),
                    detail: None,
                    status: None,
                    title: None,
                    type_: None,
                };
            }
            404 => {
                // Parse error body for NotFoundError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::NotFoundError {
                            message: crate::ext::error_message(status_code, Some(&parsed)),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::NotFoundError {
                    message: crate::ext::error_message(status_code, None),
                    detail: None,
                    status: None,
                    title: None,
                    type_: None,
                };
            }
            422 => {
                // Parse error body for UnprocessableEntityError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::UnprocessableEntityError {
                            message: crate::ext::error_message(status_code, Some(&parsed)),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::UnprocessableEntityError {
                    message: crate::ext::error_message(status_code, None),
                    detail: None,
                    status: None,
                    title: None,
                    type_: None,
                };
            }
            500 => {
                // Parse error body for InternalServerError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::InternalServerError {
                            message: crate::ext::error_message(status_code, Some(&parsed)),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::InternalServerError {
                    message: crate::ext::error_message(status_code, None),
                    detail: None,
                    status: None,
                    title: None,
                    type_: None,
                };
            }
            502 => {
                // Parse error body for BadGatewayError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::BadGatewayError {
                            message: crate::ext::error_message(status_code, Some(&parsed)),
                        };
                    }
                }
                return Self::BadGatewayError {
                    message: crate::ext::error_message(status_code, None),
                };
            }
            503 => {
                // Parse error body for ServiceUnavailableError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ServiceUnavailableError {
                            message: crate::ext::error_message(status_code, Some(&parsed)),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::ServiceUnavailableError {
                    message: crate::ext::error_message(status_code, None),
                    detail: None,
                    status: None,
                    title: None,
                    type_: None,
                };
            }
            504 => {
                // Parse error body for GatewayTimeoutError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::GatewayTimeoutError {
                            message: crate::ext::error_message(status_code, Some(&parsed)),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::GatewayTimeoutError {
                    message: crate::ext::error_message(status_code, None),
                    detail: None,
                    status: None,
                    title: None,
                    type_: None,
                };
            }
            409 => {
                // Parse error body for ConflictError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ConflictError {
                            message: crate::ext::error_message(status_code, Some(&parsed)),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::ConflictError {
                    message: crate::ext::error_message(status_code, None),
                    detail: None,
                    status: None,
                    title: None,
                    type_: None,
                };
            }
            429 => {
                // Parse error body for TooManyRequestsError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::TooManyRequestsError {
                            message: crate::ext::error_message(status_code, Some(&parsed)),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::TooManyRequestsError {
                    message: crate::ext::error_message(status_code, None),
                    detail: None,
                    status: None,
                    title: None,
                    type_: None,
                };
            }
            _ => Self::Http {
                status: status_code,
                message: crate::ext::error_message(
                    status_code,
                    body.and_then(|body| serde_json::from_str(body).ok())
                        .as_ref(),
                ),
            },
        }
    }
}

/// Error returned when a required field was not set on a builder.
#[derive(Debug)]
pub struct BuildError {
    field: &'static str,
}

impl BuildError {
    pub fn missing_field(field: &'static str) -> Self {
        Self { field }
    }
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}` was not set but is required", self.field)
    }
}

impl std::error::Error for BuildError {}
