pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckoutSessionResponse {
    /// Total amount for the session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// Whether the session charges the customer or only stores a payment method
    pub checkout_intent: CheckoutSessionResponseCheckoutIntent,
    /// Your reference supplied at creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<String>,
    /// Timestamp when the session was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// ISO 4217 currency code
    pub currency: String,
    /// Customer the checkout is for, once known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Timestamp when the session token expires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// Unique identifier for the checkout session
    pub id: String,
    /// Invoice created by the session, once completed. One-off sessions (line_items or amount) produce a paid invoice instead of a subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Payment created by the session, once completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    /// Browser-safe token that scopes the embedded checkout to this session. Pass it to the Alguna Checkout SDK.
    pub session_token: String,
    /// Checkout session status
    pub status: CheckoutSessionResponseStatus,
    /// Subscription created by the session, once completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    /// Hosted checkout URL for this session. The Alguna Checkout SDK embeds this URL; it can also be used directly as a redirect target.
    pub url: String,
}

impl CheckoutSessionResponse {
    pub fn builder() -> CheckoutSessionResponseBuilder {
        <CheckoutSessionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionResponseBuilder {
    amount: Option<String>,
    checkout_intent: Option<CheckoutSessionResponseCheckoutIntent>,
    client_reference_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    customer_id: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    invoice_id: Option<String>,
    payment_id: Option<String>,
    session_token: Option<String>,
    status: Option<CheckoutSessionResponseStatus>,
    subscription_id: Option<String>,
    url: Option<String>,
}

impl CheckoutSessionResponseBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn checkout_intent(mut self, value: CheckoutSessionResponseCheckoutIntent) -> Self {
        self.checkout_intent = Some(value);
        self
    }

    pub fn client_reference_id(mut self, value: impl Into<String>) -> Self {
        self.client_reference_id = Some(value.into());
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

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn session_token(mut self, value: impl Into<String>) -> Self {
        self.session_token = Some(value.into());
        self
    }

    pub fn status(mut self, value: CheckoutSessionResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`checkout_intent`](CheckoutSessionResponseBuilder::checkout_intent)
    /// - [`created_at`](CheckoutSessionResponseBuilder::created_at)
    /// - [`currency`](CheckoutSessionResponseBuilder::currency)
    /// - [`id`](CheckoutSessionResponseBuilder::id)
    /// - [`session_token`](CheckoutSessionResponseBuilder::session_token)
    /// - [`status`](CheckoutSessionResponseBuilder::status)
    /// - [`url`](CheckoutSessionResponseBuilder::url)
    pub fn build(self) -> Result<CheckoutSessionResponse, BuildError> {
        Ok(CheckoutSessionResponse {
            amount: self.amount,
            checkout_intent: self
                .checkout_intent
                .ok_or_else(|| BuildError::missing_field("checkout_intent"))?,
            client_reference_id: self.client_reference_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            customer_id: self.customer_id,
            expires_at: self.expires_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invoice_id: self.invoice_id,
            payment_id: self.payment_id,
            session_token: self
                .session_token
                .ok_or_else(|| BuildError::missing_field("session_token"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            subscription_id: self.subscription_id,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
