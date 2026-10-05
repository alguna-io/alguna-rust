pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscriptionResponse {
    /// When the subscription was activated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_at: Option<DateTime<FixedOffset>>,
    /// Billing configuration
    pub billing: BillingResponse,
    /// Contract configuration
    pub contract: ContractResponse,
    /// When the subscription was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// ISO 4217 currency code
    pub currency: String,
    /// Current active version ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version_id: Option<String>,
    /// Customer ID
    pub customer_id: String,
    /// Subscription-level discount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<DiscountResponse>,
    /// Unique identifier
    pub id: String,
    /// Maximum spend threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_spend: Option<SpendingThresholdResponse>,
    /// Key-value metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Minimum spend threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_spend: Option<SpendingThresholdResponse>,
    /// Subscription name
    pub name: String,
    /// Pending version changes
    pub pending_changes: Vec<PendingChangeResponse>,
    /// Plan ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// Price escalation configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_escalation: Option<PriceEscalationResponse>,
    /// Renewal configuration
    pub renewal: RenewalResponse,
    /// Subscription status
    pub status: SubscriptionResponseStatus,
    /// Trial period in days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    /// When the subscription was last updated
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl SubscriptionResponse {
    pub fn builder() -> SubscriptionResponseBuilder {
        <SubscriptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionResponseBuilder {
    activated_at: Option<DateTime<FixedOffset>>,
    billing: Option<BillingResponse>,
    contract: Option<ContractResponse>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    current_version_id: Option<String>,
    customer_id: Option<String>,
    discount: Option<DiscountResponse>,
    id: Option<String>,
    maximum_spend: Option<SpendingThresholdResponse>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    minimum_spend: Option<SpendingThresholdResponse>,
    name: Option<String>,
    pending_changes: Option<Vec<PendingChangeResponse>>,
    plan_id: Option<String>,
    price_escalation: Option<PriceEscalationResponse>,
    renewal: Option<RenewalResponse>,
    status: Option<SubscriptionResponseStatus>,
    trial_period_days: Option<i64>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl SubscriptionResponseBuilder {
    pub fn activated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.activated_at = Some(value);
        self
    }

    pub fn billing(mut self, value: BillingResponse) -> Self {
        self.billing = Some(value);
        self
    }

    pub fn contract(mut self, value: ContractResponse) -> Self {
        self.contract = Some(value);
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

    pub fn current_version_id(mut self, value: impl Into<String>) -> Self {
        self.current_version_id = Some(value.into());
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn discount(mut self, value: DiscountResponse) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn maximum_spend(mut self, value: SpendingThresholdResponse) -> Self {
        self.maximum_spend = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn minimum_spend(mut self, value: SpendingThresholdResponse) -> Self {
        self.minimum_spend = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn pending_changes(mut self, value: Vec<PendingChangeResponse>) -> Self {
        self.pending_changes = Some(value);
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn price_escalation(mut self, value: PriceEscalationResponse) -> Self {
        self.price_escalation = Some(value);
        self
    }

    pub fn renewal(mut self, value: RenewalResponse) -> Self {
        self.renewal = Some(value);
        self
    }

    pub fn status(mut self, value: SubscriptionResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn trial_period_days(mut self, value: i64) -> Self {
        self.trial_period_days = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`billing`](SubscriptionResponseBuilder::billing)
    /// - [`contract`](SubscriptionResponseBuilder::contract)
    /// - [`created_at`](SubscriptionResponseBuilder::created_at)
    /// - [`currency`](SubscriptionResponseBuilder::currency)
    /// - [`customer_id`](SubscriptionResponseBuilder::customer_id)
    /// - [`id`](SubscriptionResponseBuilder::id)
    /// - [`metadata`](SubscriptionResponseBuilder::metadata)
    /// - [`name`](SubscriptionResponseBuilder::name)
    /// - [`pending_changes`](SubscriptionResponseBuilder::pending_changes)
    /// - [`renewal`](SubscriptionResponseBuilder::renewal)
    /// - [`status`](SubscriptionResponseBuilder::status)
    /// - [`updated_at`](SubscriptionResponseBuilder::updated_at)
    pub fn build(self) -> Result<SubscriptionResponse, BuildError> {
        Ok(SubscriptionResponse {
            activated_at: self.activated_at,
            billing: self
                .billing
                .ok_or_else(|| BuildError::missing_field("billing"))?,
            contract: self
                .contract
                .ok_or_else(|| BuildError::missing_field("contract"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            current_version_id: self.current_version_id,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            discount: self.discount,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            maximum_spend: self.maximum_spend,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            minimum_spend: self.minimum_spend,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            pending_changes: self
                .pending_changes
                .ok_or_else(|| BuildError::missing_field("pending_changes"))?,
            plan_id: self.plan_id,
            price_escalation: self.price_escalation,
            renewal: self
                .renewal
                .ok_or_else(|| BuildError::missing_field("renewal"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            trial_period_days: self.trial_period_days,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
