pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubscriptionVersionPayload {
    /// Customer account the subscription belongs to
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// Unique identifier for the subscription version
    pub id: String,
    /// Version status
    pub status: SubscriptionVersionPayloadStatus,
    /// Subscription the version belongs to
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}

impl SubscriptionVersionPayload {
    pub fn builder() -> SubscriptionVersionPayloadBuilder {
        <SubscriptionVersionPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionVersionPayloadBuilder {
    account_id: Option<String>,
    id: Option<String>,
    status: Option<SubscriptionVersionPayloadStatus>,
    subscription_id: Option<String>,
}

impl SubscriptionVersionPayloadBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: SubscriptionVersionPayloadStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionVersionPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](SubscriptionVersionPayloadBuilder::account_id)
    /// - [`id`](SubscriptionVersionPayloadBuilder::id)
    /// - [`status`](SubscriptionVersionPayloadBuilder::status)
    /// - [`subscription_id`](SubscriptionVersionPayloadBuilder::subscription_id)
    pub fn build(self) -> Result<SubscriptionVersionPayload, BuildError> {
        Ok(SubscriptionVersionPayload {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            subscription_id: self
                .subscription_id
                .ok_or_else(|| BuildError::missing_field("subscription_id"))?,
        })
    }
}
