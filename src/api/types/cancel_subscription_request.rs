pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CancelSubscriptionRequest {
    /// When cancellation takes effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective: Option<String>,
    /// Cancellation reason
    pub reason: SubscriptionCancelationReason,
    /// Additional context for the cancellation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_description: Option<String>,
}

impl CancelSubscriptionRequest {
    pub fn builder() -> CancelSubscriptionRequestBuilder {
        <CancelSubscriptionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelSubscriptionRequestBuilder {
    effective: Option<String>,
    reason: Option<SubscriptionCancelationReason>,
    reason_description: Option<String>,
}

impl CancelSubscriptionRequestBuilder {
    pub fn effective(mut self, value: impl Into<String>) -> Self {
        self.effective = Some(value.into());
        self
    }

    pub fn reason(mut self, value: SubscriptionCancelationReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn reason_description(mut self, value: impl Into<String>) -> Self {
        self.reason_description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CancelSubscriptionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reason`](CancelSubscriptionRequestBuilder::reason)
    pub fn build(self) -> Result<CancelSubscriptionRequest, BuildError> {
        Ok(CancelSubscriptionRequest {
            effective: self.effective,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            reason_description: self.reason_description,
        })
    }
}
