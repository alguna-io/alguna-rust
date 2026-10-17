pub use crate::prelude::*;

/// An approver rejected their approval request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookApprovalRequestRejectedPayload {
    pub data: ApprovalRequestPayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookApprovalRequestRejectedPayload {
    pub fn builder() -> WebhookApprovalRequestRejectedPayloadBuilder {
        <WebhookApprovalRequestRejectedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookApprovalRequestRejectedPayloadBuilder {
    data: Option<ApprovalRequestPayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookApprovalRequestRejectedPayloadBuilder {
    pub fn data(mut self, value: ApprovalRequestPayload) -> Self {
        self.data = Some(value);
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebhookApprovalRequestRejectedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookApprovalRequestRejectedPayloadBuilder::data)
    /// - [`timestamp`](WebhookApprovalRequestRejectedPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookApprovalRequestRejectedPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookApprovalRequestRejectedPayload, BuildError> {
        Ok(WebhookApprovalRequestRejectedPayload {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            timestamp: self
                .timestamp
                .ok_or_else(|| BuildError::missing_field("timestamp"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
