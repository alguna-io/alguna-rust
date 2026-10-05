pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckoutSessionPayload {
    /// Customer account the checkout belongs to
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// Amount received, formatted to the currency's precision
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// The reference you supplied when creating the session
    #[serde(rename = "clientReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<String>,
    /// ISO 4217 currency code
    pub currency: String,
    /// Unique identifier for the checkout session
    pub id: String,
    /// Invoice paid by this checkout. Set for one-off purchases, null for plan checkouts.
    #[serde(rename = "invoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Token echoed from the customer session that opened the checkout
    #[serde(rename = "passThroughToken")]
    pub pass_through_token: String,
    /// Payment recorded for this checkout
    #[serde(rename = "paymentId")]
    pub payment_id: String,
    /// Checkout session status
    pub status: String,
    /// Subscription created by this checkout. Set for plan checkouts, null for one-off purchases.
    #[serde(rename = "subscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}

impl CheckoutSessionPayload {
    pub fn builder() -> CheckoutSessionPayloadBuilder {
        <CheckoutSessionPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionPayloadBuilder {
    account_id: Option<String>,
    amount: Option<String>,
    client_reference_id: Option<String>,
    currency: Option<String>,
    id: Option<String>,
    invoice_id: Option<String>,
    pass_through_token: Option<String>,
    payment_id: Option<String>,
    status: Option<String>,
    subscription_id: Option<String>,
}

impl CheckoutSessionPayloadBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn pass_through_token(mut self, value: impl Into<String>) -> Self {
        self.pass_through_token = Some(value.into());
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CheckoutSessionPayloadBuilder::account_id)
    /// - [`currency`](CheckoutSessionPayloadBuilder::currency)
    /// - [`id`](CheckoutSessionPayloadBuilder::id)
    /// - [`pass_through_token`](CheckoutSessionPayloadBuilder::pass_through_token)
    /// - [`payment_id`](CheckoutSessionPayloadBuilder::payment_id)
    /// - [`status`](CheckoutSessionPayloadBuilder::status)
    pub fn build(self) -> Result<CheckoutSessionPayload, BuildError> {
        Ok(CheckoutSessionPayload {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            amount: self.amount,
            client_reference_id: self.client_reference_id,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invoice_id: self.invoice_id,
            pass_through_token: self
                .pass_through_token
                .ok_or_else(|| BuildError::missing_field("pass_through_token"))?,
            payment_id: self
                .payment_id
                .ok_or_else(|| BuildError::missing_field("payment_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            subscription_id: self.subscription_id,
        })
    }
}
