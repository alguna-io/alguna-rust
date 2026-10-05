pub use crate::prelude::*;

/// Approval was requested from an approver. One event is sent per approver.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookApprovalRequestCreatedPayload {
    pub data: ApprovalRequestPayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookApprovalRequestCreatedPayload {
    pub fn builder() -> WebhookApprovalRequestCreatedPayloadBuilder {
        <WebhookApprovalRequestCreatedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookApprovalRequestCreatedPayloadBuilder {
    data: Option<ApprovalRequestPayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookApprovalRequestCreatedPayloadBuilder {
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

    /// Consumes the builder and constructs a [`WebhookApprovalRequestCreatedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookApprovalRequestCreatedPayloadBuilder::data)
    /// - [`timestamp`](WebhookApprovalRequestCreatedPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookApprovalRequestCreatedPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookApprovalRequestCreatedPayload, BuildError> {
        Ok(WebhookApprovalRequestCreatedPayload {
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
