pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckoutSessionCreateRequest {
    /// Total amount to charge when neither plan_id nor line_items are provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// payment charges the customer; vault only collects and stores a payment method. Defaults to payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_intent: Option<CheckoutSessionCreateRequestCheckoutIntent>,
    /// Your own reference for this checkout (order ID, cart ID). Echoed on the session and the checkout.session.completed webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<String>,
    /// ISO 4217 currency code; defaults to the organization currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Details the customer account is created from when checkout completes. When neither this nor customer_id/customer_alias is provided, the embedded checkout collects them from the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CheckoutCustomerRequest>,
    /// Alternative identifier (external ID) for the customer. Used when customer_id is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_alias: Option<String>,
    /// Customer the checkout is for. Omit for guest checkout and pass customer instead — the customer account is created when checkout completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Session lifetime in seconds (5 minutes to 7 days). Defaults to 86400 (24 hours).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_seconds: Option<i64>,
    /// Legal entity to bill from. Defaults to the customer's legal entity, falling back to the organization default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_entity_id: Option<String>,
    /// Ad-hoc line items to charge instead of a plan. Mutually exclusive with plan_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<CheckoutLineItemRequest>>,
    /// Plan to check out. The session's line items and amount are derived from the plan's prices. Mutually exclusive with line_items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// URL the customer can be sent to after a successful checkout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
}

impl CheckoutSessionCreateRequest {
    pub fn builder() -> CheckoutSessionCreateRequestBuilder {
        <CheckoutSessionCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionCreateRequestBuilder {
    amount: Option<String>,
    checkout_intent: Option<CheckoutSessionCreateRequestCheckoutIntent>,
    client_reference_id: Option<String>,
    currency: Option<String>,
    customer: Option<CheckoutCustomerRequest>,
    customer_alias: Option<String>,
    customer_id: Option<String>,
    expires_in_seconds: Option<i64>,
    legal_entity_id: Option<String>,
    line_items: Option<Vec<CheckoutLineItemRequest>>,
    plan_id: Option<String>,
    success_url: Option<String>,
}

impl CheckoutSessionCreateRequestBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn checkout_intent(mut self, value: CheckoutSessionCreateRequestCheckoutIntent) -> Self {
        self.checkout_intent = Some(value);
        self
    }

    pub fn client_reference_id(mut self, value: impl Into<String>) -> Self {
        self.client_reference_id = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn customer(mut self, value: CheckoutCustomerRequest) -> Self {
        self.customer = Some(value);
        self
    }

    pub fn customer_alias(mut self, value: impl Into<String>) -> Self {
        self.customer_alias = Some(value.into());
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn expires_in_seconds(mut self, value: i64) -> Self {
        self.expires_in_seconds = Some(value);
        self
    }

    pub fn legal_entity_id(mut self, value: impl Into<String>) -> Self {
        self.legal_entity_id = Some(value.into());
        self
    }

    pub fn line_items(mut self, value: Vec<CheckoutLineItemRequest>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn success_url(mut self, value: impl Into<String>) -> Self {
        self.success_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionCreateRequest`].
    pub fn build(self) -> Result<CheckoutSessionCreateRequest, BuildError> {
        Ok(CheckoutSessionCreateRequest {
            amount: self.amount,
            checkout_intent: self.checkout_intent,
            client_reference_id: self.client_reference_id,
            currency: self.currency,
            customer: self.customer,
            customer_alias: self.customer_alias,
            customer_id: self.customer_id,
            expires_in_seconds: self.expires_in_seconds,
            legal_entity_id: self.legal_entity_id,
            line_items: self.line_items,
            plan_id: self.plan_id,
            success_url: self.success_url,
        })
    }
}
