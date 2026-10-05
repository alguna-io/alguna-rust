pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateSubscriptionRequest {
    /// Billing configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<UpdateBillingInput>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Subscription-level discount (percentage only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<DiscountInput>,
    /// Maximum spend threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_spend: Option<SpendingThresholdInput>,
    /// Key-value metadata (null value removes key)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Minimum spend threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_spend: Option<SpendingThresholdInput>,
    /// Subscription name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Price escalation config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_escalation: Option<PriceEscalationInput>,
    /// Renewal configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal: Option<RenewalInput>,
    /// Tag IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Trial period in days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
}

impl UpdateSubscriptionRequest {
    pub fn builder() -> UpdateSubscriptionRequestBuilder {
        <UpdateSubscriptionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSubscriptionRequestBuilder {
    billing: Option<UpdateBillingInput>,
    description: Option<String>,
    discount: Option<DiscountInput>,
    maximum_spend: Option<SpendingThresholdInput>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    minimum_spend: Option<SpendingThresholdInput>,
    name: Option<String>,
    price_escalation: Option<PriceEscalationInput>,
    renewal: Option<RenewalInput>,
    tags: Option<Vec<String>>,
    trial_period_days: Option<i64>,
}

impl UpdateSubscriptionRequestBuilder {
    pub fn billing(mut self, value: UpdateBillingInput) -> Self {
        self.billing = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn discount(mut self, value: DiscountInput) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn maximum_spend(mut self, value: SpendingThresholdInput) -> Self {
        self.maximum_spend = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn minimum_spend(mut self, value: SpendingThresholdInput) -> Self {
        self.minimum_spend = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn price_escalation(mut self, value: PriceEscalationInput) -> Self {
        self.price_escalation = Some(value);
        self
    }

    pub fn renewal(mut self, value: RenewalInput) -> Self {
        self.renewal = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn trial_period_days(mut self, value: i64) -> Self {
        self.trial_period_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateSubscriptionRequest`].
    pub fn build(self) -> Result<UpdateSubscriptionRequest, BuildError> {
        Ok(UpdateSubscriptionRequest {
            billing: self.billing,
            description: self.description,
            discount: self.discount,
            maximum_spend: self.maximum_spend,
            metadata: self.metadata,
            minimum_spend: self.minimum_spend,
            name: self.name,
            price_escalation: self.price_escalation,
            renewal: self.renewal,
            tags: self.tags,
            trial_period_days: self.trial_period_days,
        })
    }
}
