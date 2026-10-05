pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreditDebitWalletRequest {
    /// Amount to credit or debit
    pub amount: String,
    /// Description of the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Reference identifier for the transaction
    pub reference_id: String,
    /// Type of transaction
    pub transaction_type: LedgerEntryTransactionType,
}

impl CreditDebitWalletRequest {
    pub fn builder() -> CreditDebitWalletRequestBuilder {
        <CreditDebitWalletRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditDebitWalletRequestBuilder {
    amount: Option<String>,
    description: Option<String>,
    reference_id: Option<String>,
    transaction_type: Option<LedgerEntryTransactionType>,
}

impl CreditDebitWalletRequestBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn reference_id(mut self, value: impl Into<String>) -> Self {
        self.reference_id = Some(value.into());
        self
    }

    pub fn transaction_type(mut self, value: LedgerEntryTransactionType) -> Self {
        self.transaction_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreditDebitWalletRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CreditDebitWalletRequestBuilder::amount)
    /// - [`reference_id`](CreditDebitWalletRequestBuilder::reference_id)
    /// - [`transaction_type`](CreditDebitWalletRequestBuilder::transaction_type)
    pub fn build(self) -> Result<CreditDebitWalletRequest, BuildError> {
        Ok(CreditDebitWalletRequest {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            description: self.description,
            reference_id: self
                .reference_id
                .ok_or_else(|| BuildError::missing_field("reference_id"))?,
            transaction_type: self
                .transaction_type
                .ok_or_else(|| BuildError::missing_field("transaction_type"))?,
        })
    }
}
