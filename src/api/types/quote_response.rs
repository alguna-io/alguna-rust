pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QuoteResponse {
    /// Whether this quote takes effect on its own once signed; otherwise activate it with Activate a Signed Quote
    pub auto_activate: bool,
    /// Term of the subscription this quote documents
    pub contract: ContractResponse,
    /// When the quote was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// ISO 4217 currency code
    pub currency: String,
    /// Customer ID
    pub customer_id: String,
    /// When the signing offer expires; null means it never expires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// Quote ID
    pub id: String,
    /// Quote kind
    pub kind: QuoteResponseKind,
    /// Display name of the quote: its own override when one is set, otherwise the name of the subscription it documents
    pub name: String,
    /// Hosted page where the customer reviews and signs. Present on a single quote while a customer signature is pending, so a lost link can be recovered; omitted on the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_url: Option<String>,
    /// Document status
    pub status: QuoteResponseStatus,
    /// The subscription this quote creates or amends. Whether the deal is in effect is that subscription's state, not the quote's: read it, or its phases, once the quote is signed.
    pub subscription_id: String,
    /// When the quote was last updated
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Reason recorded on withdrawal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawal_reason: Option<String>,
    /// When the quote was withdrawn, if withdrawn
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawn_at: Option<DateTime<FixedOffset>>,
}

impl QuoteResponse {
    pub fn builder() -> QuoteResponseBuilder {
        <QuoteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuoteResponseBuilder {
    auto_activate: Option<bool>,
    contract: Option<ContractResponse>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    customer_id: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    kind: Option<QuoteResponseKind>,
    name: Option<String>,
    signing_url: Option<String>,
    status: Option<QuoteResponseStatus>,
    subscription_id: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    withdrawal_reason: Option<String>,
    withdrawn_at: Option<DateTime<FixedOffset>>,
}

impl QuoteResponseBuilder {
    pub fn auto_activate(mut self, value: bool) -> Self {
        self.auto_activate = Some(value);
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

    pub fn kind(mut self, value: QuoteResponseKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn signing_url(mut self, value: impl Into<String>) -> Self {
        self.signing_url = Some(value.into());
        self
    }

    pub fn status(mut self, value: QuoteResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn withdrawal_reason(mut self, value: impl Into<String>) -> Self {
        self.withdrawal_reason = Some(value.into());
        self
    }

    pub fn withdrawn_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.withdrawn_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuoteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auto_activate`](QuoteResponseBuilder::auto_activate)
    /// - [`contract`](QuoteResponseBuilder::contract)
    /// - [`created_at`](QuoteResponseBuilder::created_at)
    /// - [`currency`](QuoteResponseBuilder::currency)
    /// - [`customer_id`](QuoteResponseBuilder::customer_id)
    /// - [`id`](QuoteResponseBuilder::id)
    /// - [`kind`](QuoteResponseBuilder::kind)
    /// - [`name`](QuoteResponseBuilder::name)
    /// - [`status`](QuoteResponseBuilder::status)
    /// - [`subscription_id`](QuoteResponseBuilder::subscription_id)
    /// - [`updated_at`](QuoteResponseBuilder::updated_at)
    pub fn build(self) -> Result<QuoteResponse, BuildError> {
        Ok(QuoteResponse {
            auto_activate: self
                .auto_activate
                .ok_or_else(|| BuildError::missing_field("auto_activate"))?,
            contract: self
                .contract
                .ok_or_else(|| BuildError::missing_field("contract"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            expires_at: self.expires_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            signing_url: self.signing_url,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            subscription_id: self
                .subscription_id
                .ok_or_else(|| BuildError::missing_field("subscription_id"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            withdrawal_reason: self.withdrawal_reason,
            withdrawn_at: self.withdrawn_at,
        })
    }
}
