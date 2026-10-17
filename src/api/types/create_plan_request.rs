pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePlanRequest {
    /// Markdown additional terms appended to the contract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_terms: Option<String>,
    /// Auto-activate subscriptions created from this plan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_activate: Option<bool>,
    /// Whether the plan auto-renews
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// Contract duration in months
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_duration: Option<i64>,
    /// Contract period type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_period_type: Option<DurationType>,
    /// ISO 4217 currency code
    pub currency: String,
    /// Plan description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Plan-level discount applied across the price set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<DiscountInput>,
    /// Entitlement configs to provision with the plan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<EntitlementInput>>,
    /// Invoice/email behavior overrides for the plan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreatePlanInvoiceSettingsInput>,
    /// Items that make up the plan's pricing template. Each item is either a standalone product (product_id + price) or a bundle (bundle_id + name + items).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CreatePlanItemInput>>,
    /// Maximum spend per period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_spend: Option<SpendingThresholdInput>,
    /// Minimum spend per period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_spend: Option<SpendingThresholdInput>,
    /// Plan name
    pub name: String,
    /// Invoice payment terms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<String>,
    /// Price escalation config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_escalation: Option<PriceEscalationInput>,
    /// Renewal duration in months
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_duration: Option<i64>,
    /// Renewal period type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_period_type: Option<DurationType>,
    /// Tag IDs to associate with the plan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Trial period in days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
}

impl CreatePlanRequest {
    pub fn builder() -> CreatePlanRequestBuilder {
        <CreatePlanRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePlanRequestBuilder {
    additional_terms: Option<String>,
    auto_activate: Option<bool>,
    auto_renew: Option<bool>,
    contract_duration: Option<i64>,
    contract_period_type: Option<DurationType>,
    currency: Option<String>,
    description: Option<String>,
    discount: Option<DiscountInput>,
    entitlements: Option<Vec<EntitlementInput>>,
    invoice_settings: Option<CreatePlanInvoiceSettingsInput>,
    items: Option<Vec<CreatePlanItemInput>>,
    maximum_spend: Option<SpendingThresholdInput>,
    minimum_spend: Option<SpendingThresholdInput>,
    name: Option<String>,
    payment_terms: Option<String>,
    price_escalation: Option<PriceEscalationInput>,
    renewal_duration: Option<i64>,
    renewal_period_type: Option<DurationType>,
    tags: Option<Vec<String>>,
    trial_period_days: Option<i64>,
}

impl CreatePlanRequestBuilder {
    pub fn additional_terms(mut self, value: impl Into<String>) -> Self {
        self.additional_terms = Some(value.into());
        self
    }

    pub fn auto_activate(mut self, value: bool) -> Self {
        self.auto_activate = Some(value);
        self
    }

    pub fn auto_renew(mut self, value: bool) -> Self {
        self.auto_renew = Some(value);
        self
    }

    pub fn contract_duration(mut self, value: i64) -> Self {
        self.contract_duration = Some(value);
        self
    }

    pub fn contract_period_type(mut self, value: DurationType) -> Self {
        self.contract_period_type = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
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

    pub fn entitlements(mut self, value: Vec<EntitlementInput>) -> Self {
        self.entitlements = Some(value);
        self
    }

    pub fn invoice_settings(mut self, value: CreatePlanInvoiceSettingsInput) -> Self {
        self.invoice_settings = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<CreatePlanItemInput>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn maximum_spend(mut self, value: SpendingThresholdInput) -> Self {
        self.maximum_spend = Some(value);
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

    pub fn payment_terms(mut self, value: impl Into<String>) -> Self {
        self.payment_terms = Some(value.into());
        self
    }

    pub fn price_escalation(mut self, value: PriceEscalationInput) -> Self {
        self.price_escalation = Some(value);
        self
    }

    pub fn renewal_duration(mut self, value: i64) -> Self {
        self.renewal_duration = Some(value);
        self
    }

    pub fn renewal_period_type(mut self, value: DurationType) -> Self {
        self.renewal_period_type = Some(value);
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

    /// Consumes the builder and constructs a [`CreatePlanRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](CreatePlanRequestBuilder::currency)
    /// - [`name`](CreatePlanRequestBuilder::name)
    pub fn build(self) -> Result<CreatePlanRequest, BuildError> {
        Ok(CreatePlanRequest {
            additional_terms: self.additional_terms,
            auto_activate: self.auto_activate,
            auto_renew: self.auto_renew,
            contract_duration: self.contract_duration,
            contract_period_type: self.contract_period_type,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            description: self.description,
            discount: self.discount,
            entitlements: self.entitlements,
            invoice_settings: self.invoice_settings,
            items: self.items,
            maximum_spend: self.maximum_spend,
            minimum_spend: self.minimum_spend,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            payment_terms: self.payment_terms,
            price_escalation: self.price_escalation,
            renewal_duration: self.renewal_duration,
            renewal_period_type: self.renewal_period_type,
            tags: self.tags,
            trial_period_days: self.trial_period_days,
        })
    }
}
