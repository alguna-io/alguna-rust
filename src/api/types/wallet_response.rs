pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WalletResponse {
    /// Whether the wallet is active
    pub active: bool,
    /// Timestamp when the wallet was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Currency code
    pub currency: String,
    /// Current balance of the wallet
    pub current_balance: String,
    /// Identifier of the customer that owns this wallet
    pub customer_id: String,
    /// Unique identifier for the wallet
    pub id: String,
    /// Name of the wallet
    pub name: String,
    /// Product IDs that can be purchased with this wallet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_ids: Option<Vec<String>>,
    /// Timestamp when the wallet was last updated
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl WalletResponse {
    pub fn builder() -> WalletResponseBuilder {
        <WalletResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WalletResponseBuilder {
    active: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    current_balance: Option<String>,
    customer_id: Option<String>,
    id: Option<String>,
    name: Option<String>,
    product_ids: Option<Vec<String>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl WalletResponseBuilder {
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
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

    pub fn current_balance(mut self, value: impl Into<String>) -> Self {
        self.current_balance = Some(value.into());
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
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

    pub fn product_ids(mut self, value: Vec<String>) -> Self {
        self.product_ids = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WalletResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`active`](WalletResponseBuilder::active)
    /// - [`created_at`](WalletResponseBuilder::created_at)
    /// - [`currency`](WalletResponseBuilder::currency)
    /// - [`current_balance`](WalletResponseBuilder::current_balance)
    /// - [`customer_id`](WalletResponseBuilder::customer_id)
    /// - [`id`](WalletResponseBuilder::id)
    /// - [`name`](WalletResponseBuilder::name)
    /// - [`updated_at`](WalletResponseBuilder::updated_at)
    pub fn build(self) -> Result<WalletResponse, BuildError> {
        Ok(WalletResponse {
            active: self
                .active
                .ok_or_else(|| BuildError::missing_field("active"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            current_balance: self
                .current_balance
                .ok_or_else(|| BuildError::missing_field("current_balance"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            product_ids: self.product_ids,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
