pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateSubscriptionRequest {
    /// Auto-activate after creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_activate: Option<bool>,
    /// Billing configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<BillingInput>,
    /// Contract configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<ContractInput>,
    /// Currency code. Required when plan_id is not provided. When plan_id is set, defaults to the customer's currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Customer ID
    pub customer_id: String,
    /// Items for the first subscription version. Without plan_id, this is the full set of items. With plan_id, items override matching plan items (by product_id or bundle_id) and non-matching items are appended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CreateSubscriptionItemInput>>,
    /// Key-value metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Subscription name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Plan ID to use as the base item set (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// Renewal configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal: Option<RenewalInput>,
}

impl CreateSubscriptionRequest {
    pub fn builder() -> CreateSubscriptionRequestBuilder {
        <CreateSubscriptionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSubscriptionRequestBuilder {
    auto_activate: Option<bool>,
    billing: Option<BillingInput>,
    contract: Option<ContractInput>,
    currency: Option<String>,
    customer_id: Option<String>,
    items: Option<Vec<CreateSubscriptionItemInput>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    name: Option<String>,
    plan_id: Option<String>,
    renewal: Option<RenewalInput>,
}

impl CreateSubscriptionRequestBuilder {
    pub fn auto_activate(mut self, value: bool) -> Self {
        self.auto_activate = Some(value);
        self
    }

    pub fn billing(mut self, value: BillingInput) -> Self {
        self.billing = Some(value);
        self
    }

    pub fn contract(mut self, value: ContractInput) -> Self {
        self.contract = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<CreateSubscriptionItemInput>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn renewal(mut self, value: RenewalInput) -> Self {
        self.renewal = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSubscriptionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer_id`](CreateSubscriptionRequestBuilder::customer_id)
    pub fn build(self) -> Result<CreateSubscriptionRequest, BuildError> {
        Ok(CreateSubscriptionRequest {
            auto_activate: self.auto_activate,
            billing: self.billing,
            contract: self.contract,
            currency: self.currency,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            items: self.items,
            metadata: self.metadata,
            name: self.name,
            plan_id: self.plan_id,
            renewal: self.renewal,
        })
    }
}
