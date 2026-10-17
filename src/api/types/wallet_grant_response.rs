pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WalletGrantResponse {
    /// Amount of the grant
    pub amount: String,
    /// Timestamp when the grant was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Description of the grant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When the grant expires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// Unique identifier for the wallet grant
    pub id: String,
    /// Associated invoice identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Associated payment identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    /// Status of the grant
    pub status: WalletGrantStatus,
    /// Timestamp when the grant was last updated
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Identifier of the wallet this grant belongs to
    pub wallet_id: String,
}

impl WalletGrantResponse {
    pub fn builder() -> WalletGrantResponseBuilder {
        <WalletGrantResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WalletGrantResponseBuilder {
    amount: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    invoice_id: Option<String>,
    payment_id: Option<String>,
    status: Option<WalletGrantStatus>,
    updated_at: Option<DateTime<FixedOffset>>,
    wallet_id: Option<String>,
}

impl WalletGrantResponseBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
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

    pub fn status(mut self, value: WalletGrantStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn wallet_id(mut self, value: impl Into<String>) -> Self {
        self.wallet_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WalletGrantResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](WalletGrantResponseBuilder::amount)
    /// - [`created_at`](WalletGrantResponseBuilder::created_at)
    /// - [`id`](WalletGrantResponseBuilder::id)
    /// - [`status`](WalletGrantResponseBuilder::status)
    /// - [`updated_at`](WalletGrantResponseBuilder::updated_at)
    /// - [`wallet_id`](WalletGrantResponseBuilder::wallet_id)
    pub fn build(self) -> Result<WalletGrantResponse, BuildError> {
        Ok(WalletGrantResponse {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self.description,
            expires_at: self.expires_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invoice_id: self.invoice_id,
            payment_id: self.payment_id,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            wallet_id: self
                .wallet_id
                .ok_or_else(|| BuildError::missing_field("wallet_id"))?,
        })
    }
}
