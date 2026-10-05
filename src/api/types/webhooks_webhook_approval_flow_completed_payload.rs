pub use crate::prelude::*;

/// A quote's approval flow reached a terminal state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookApprovalFlowCompletedPayload {
    pub data: ApprovalFlowPayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookApprovalFlowCompletedPayload {
    pub fn builder() -> WebhookApprovalFlowCompletedPayloadBuilder {
        <WebhookApprovalFlowCompletedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookApprovalFlowCompletedPayloadBuilder {
    data: Option<ApprovalFlowPayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookApprovalFlowCompletedPayloadBuilder {
    pub fn data(mut self, value: ApprovalFlowPayload) -> Self {
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

    /// Consumes the builder and constructs a [`WebhookApprovalFlowCompletedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookApprovalFlowCompletedPayloadBuilder::data)
    /// - [`timestamp`](WebhookApprovalFlowCompletedPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookApprovalFlowCompletedPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookApprovalFlowCompletedPayload, BuildError> {
        Ok(WebhookApprovalFlowCompletedPayload {
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
