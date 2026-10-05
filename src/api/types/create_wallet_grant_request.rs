pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWalletGrantRequest {
    /// Amount of the grant
    pub amount: String,
    /// Description of the grant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When the grant expires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// Associated payment identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    /// Identifier of the wallet
    pub wallet_id: String,
}

impl CreateWalletGrantRequest {
    pub fn builder() -> CreateWalletGrantRequestBuilder {
        <CreateWalletGrantRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWalletGrantRequestBuilder {
    amount: Option<String>,
    description: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    payment_id: Option<String>,
    wallet_id: Option<String>,
}

impl CreateWalletGrantRequestBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
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

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn wallet_id(mut self, value: impl Into<String>) -> Self {
        self.wallet_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateWalletGrantRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CreateWalletGrantRequestBuilder::amount)
    /// - [`wallet_id`](CreateWalletGrantRequestBuilder::wallet_id)
    pub fn build(self) -> Result<CreateWalletGrantRequest, BuildError> {
        Ok(CreateWalletGrantRequest {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            description: self.description,
            expires_at: self.expires_at,
            payment_id: self.payment_id,
            wallet_id: self
                .wallet_id
                .ok_or_else(|| BuildError::missing_field("wallet_id"))?,
        })
    }
}
