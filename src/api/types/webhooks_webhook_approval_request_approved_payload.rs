pub use crate::prelude::*;

/// An approver approved their approval request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookApprovalRequestApprovedPayload {
    pub data: ApprovalRequestPayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookApprovalRequestApprovedPayload {
    pub fn builder() -> WebhookApprovalRequestApprovedPayloadBuilder {
        <WebhookApprovalRequestApprovedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookApprovalRequestApprovedPayloadBuilder {
    data: Option<ApprovalRequestPayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookApprovalRequestApprovedPayloadBuilder {
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

    /// Consumes the builder and constructs a [`WebhookApprovalRequestApprovedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookApprovalRequestApprovedPayloadBuilder::data)
    /// - [`timestamp`](WebhookApprovalRequestApprovedPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookApprovalRequestApprovedPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookApprovalRequestApprovedPayload, BuildError> {
        Ok(WebhookApprovalRequestApprovedPayload {
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
