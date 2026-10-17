pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreditBalanceScope {
    /// Reserved. A grant's product/price filters are applied when credits are applied to an invoice, not by the gate, so this is never set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HashMap<String, serde_json::Value>>,
    /// The level this availability was resolved at. Always account: credit is resolved across every grant the account holds. A grant made under a subscription still funds realtime draws — its subscription records where it came from, and does not narrow what it can pay for — so the ledger named by an entry may belong to a subscription even though the scope is the account.
    pub level: String,
    /// Reserved. Availability is never scoped to a subscription, so this is never set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}

impl CreditBalanceScope {
    pub fn builder() -> CreditBalanceScopeBuilder {
        <CreditBalanceScopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditBalanceScopeBuilder {
    filters: Option<HashMap<String, serde_json::Value>>,
    level: Option<String>,
    subscription_id: Option<String>,
}

impl CreditBalanceScopeBuilder {
    pub fn filters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn level(mut self, value: impl Into<String>) -> Self {
        self.level = Some(value.into());
        self
    }

    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreditBalanceScope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`level`](CreditBalanceScopeBuilder::level)
    pub fn build(self) -> Result<CreditBalanceScope, BuildError> {
        Ok(CreditBalanceScope {
            filters: self.filters,
            level: self
                .level
                .ok_or_else(|| BuildError::missing_field("level"))?,
            subscription_id: self.subscription_id,
        })
    }
}
