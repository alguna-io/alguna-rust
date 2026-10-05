pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QuoteCreateRequest {
    /// Amend this active subscription. Takes exactly one entry in phases, with a start_date, and no other origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amends_subscription_id: Option<String>,
    /// Whether the subscription takes effect on its own once signed: a new-business quote activates at its contract start, an amendment applies immediately. Defaults to the plan's or organization's setting for new business, and to true for an amendment; each quote keeps its own value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_activate: Option<bool>,
    /// Term of the subscription the quote documents. Only valid for a plan-seeded quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<QuoteContractInput>,
    /// Customer ID. For an amendment, must be the subscription's customer.
    pub customer_id: String,
    /// When the signing offer lapses. Defaults to the organization's quote expiry window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// Priced items for a single-phase quote. Shorthand for one entry in phases. Prices inherit the customer's currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<VersionItemInput>>,
    /// Arbitrary key/value pairs stored on the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Names the subscription a blank-canvas quote creates. Only valid when neither plan_id nor source_subscription_id is provided; a plan or duplicate names itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Priced phases in schedule order: the ramp of a blank-canvas quote, or the one changed phase of an amendment. Mutually exclusive with items, plan_id and source_subscription_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<QuotePhaseInput>>,
    /// Seed the quote from a plan. The quote inherits the plan's pricing. Mutually exclusive with source_subscription_id and price_book_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// Price book a blank-canvas quote draws its pricing from. Mutually exclusive with plan_id and source_subscription_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_book_id: Option<String>,
    /// Duplicate an existing subscription into a new quote. Mutually exclusive with plan_id and price_book_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_subscription_id: Option<String>,
}

impl QuoteCreateRequest {
    pub fn builder() -> QuoteCreateRequestBuilder {
        <QuoteCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuoteCreateRequestBuilder {
    amends_subscription_id: Option<String>,
    auto_activate: Option<bool>,
    contract: Option<QuoteContractInput>,
    customer_id: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    items: Option<Vec<VersionItemInput>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    name: Option<String>,
    phases: Option<Vec<QuotePhaseInput>>,
    plan_id: Option<String>,
    price_book_id: Option<String>,
    source_subscription_id: Option<String>,
}

impl QuoteCreateRequestBuilder {
    pub fn amends_subscription_id(mut self, value: impl Into<String>) -> Self {
        self.amends_subscription_id = Some(value.into());
        self
    }

    pub fn auto_activate(mut self, value: bool) -> Self {
        self.auto_activate = Some(value);
        self
    }

    pub fn contract(mut self, value: QuoteContractInput) -> Self {
        self.contract = Some(value);
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<VersionItemInput>) -> Self {
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

    pub fn phases(mut self, value: Vec<QuotePhaseInput>) -> Self {
        self.phases = Some(value);
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn price_book_id(mut self, value: impl Into<String>) -> Self {
        self.price_book_id = Some(value.into());
        self
    }

    pub fn source_subscription_id(mut self, value: impl Into<String>) -> Self {
        self.source_subscription_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QuoteCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer_id`](QuoteCreateRequestBuilder::customer_id)
    pub fn build(self) -> Result<QuoteCreateRequest, BuildError> {
        Ok(QuoteCreateRequest {
            amends_subscription_id: self.amends_subscription_id,
            auto_activate: self.auto_activate,
            contract: self.contract,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            expires_at: self.expires_at,
            items: self.items,
            metadata: self.metadata,
            name: self.name,
            phases: self.phases,
            plan_id: self.plan_id,
            price_book_id: self.price_book_id,
            source_subscription_id: self.source_subscription_id,
        })
    }
}
