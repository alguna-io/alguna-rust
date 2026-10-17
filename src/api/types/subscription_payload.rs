pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubscriptionPayload {
    /// Customer account the subscription belongs to
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// When the subscription was activated, RFC 3339
    #[serde(rename = "activatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_at: Option<DateTime<FixedOffset>>,
    /// End of the contract term, RFC 3339
    #[serde(rename = "contractEndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_end_date: Option<DateTime<FixedOffset>>,
    /// Start of the contract term, RFC 3339
    #[serde(rename = "contractStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_start_date: Option<DateTime<FixedOffset>>,
    /// When the subscription was created, RFC 3339
    #[serde(rename = "createdAt")]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// ISO 4217 currency code
    pub currency: String,
    /// When the subscription ended, RFC 3339. Null while it is still running.
    #[serde(rename = "endedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<DateTime<FixedOffset>>,
    /// Unique identifier for the subscription
    pub id: String,
    /// Subscription name
    pub name: String,
    /// Subscription status
    pub status: SubscriptionPayloadStatus,
    /// When the subscription was last updated, RFC 3339
    #[serde(rename = "updatedAt")]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl SubscriptionPayload {
    pub fn builder() -> SubscriptionPayloadBuilder {
        <SubscriptionPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionPayloadBuilder {
    account_id: Option<String>,
    activated_at: Option<DateTime<FixedOffset>>,
    contract_end_date: Option<DateTime<FixedOffset>>,
    contract_start_date: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    ended_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    name: Option<String>,
    status: Option<SubscriptionPayloadStatus>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl SubscriptionPayloadBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn activated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.activated_at = Some(value);
        self
    }

    pub fn contract_end_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.contract_end_date = Some(value);
        self
    }

    pub fn contract_start_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.contract_start_date = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn ended_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.ended_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: SubscriptionPayloadStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](SubscriptionPayloadBuilder::account_id)
    /// - [`created_at`](SubscriptionPayloadBuilder::created_at)
    /// - [`currency`](SubscriptionPayloadBuilder::currency)
    /// - [`id`](SubscriptionPayloadBuilder::id)
    /// - [`name`](SubscriptionPayloadBuilder::name)
    /// - [`status`](SubscriptionPayloadBuilder::status)
    /// - [`updated_at`](SubscriptionPayloadBuilder::updated_at)
    pub fn build(self) -> Result<SubscriptionPayload, BuildError> {
        Ok(SubscriptionPayload {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            activated_at: self.activated_at,
            contract_end_date: self.contract_end_date,
            contract_start_date: self.contract_start_date,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            ended_at: self.ended_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
