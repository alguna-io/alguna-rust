pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WalletBalanceResponse {
    /// Current balance of the wallet
    pub balance: String,
}

impl WalletBalanceResponse {
    pub fn builder() -> WalletBalanceResponseBuilder {
        <WalletBalanceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WalletBalanceResponseBuilder {
    balance: Option<String>,
}

impl WalletBalanceResponseBuilder {
    pub fn balance(mut self, value: impl Into<String>) -> Self {
        self.balance = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WalletBalanceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`balance`](WalletBalanceResponseBuilder::balance)
    pub fn build(self) -> Result<WalletBalanceResponse, BuildError> {
        Ok(WalletBalanceResponse {
            balance: self
                .balance
                .ok_or_else(|| BuildError::missing_field("balance"))?,
        })
    }
}
