pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreditBalance {
    /// What these metrics can still spend from this ledger. When filtered is true this is the eligible grant balances net of open holds; otherwise it is the plain ledger figure.
    pub available: String,
    /// The credit denomination of this entry.
    pub credit_type: CreditBalanceCreditType,
    /// True when available reflects the resolved eligible grants net of open holds. False when no plan was resolved for this entry (a replay, a rate-limit shed, a settlement against a hold, or a degraded availability read) and available is the plain ledger figure.
    pub filtered: bool,
    /// The credit ledger this availability sits on. Omitted when there is no wallet to name: no plan was resolved for the entry (a replay, a rate-limit shed), no matching grant exists — none was ever made, or every one has expired or been voided — or the ledger read behind a degraded entry also failed. A merely drained wallet keeps its id, so balances[0].ledger_id still answers at the moment credit runs out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_id: Option<String>,
    /// The billable metrics this availability applies to. One metric's credit can span several ledgers, so sum its entries to reconstruct its spendability; two metrics with identical availability share one entry.
    pub metric_ids: Vec<String>,
    /// The scope the entry was resolved under. Omitted when the outcome was answered before any plan was resolved (a rate-limit shed) or against a hold's own ledgers (a settlement).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<CreditBalanceScope>,
}

impl CreditBalance {
    pub fn builder() -> CreditBalanceBuilder {
        <CreditBalanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditBalanceBuilder {
    available: Option<String>,
    credit_type: Option<CreditBalanceCreditType>,
    filtered: Option<bool>,
    ledger_id: Option<String>,
    metric_ids: Option<Vec<String>>,
    scope: Option<CreditBalanceScope>,
}

impl CreditBalanceBuilder {
    pub fn available(mut self, value: impl Into<String>) -> Self {
        self.available = Some(value.into());
        self
    }

    pub fn credit_type(mut self, value: CreditBalanceCreditType) -> Self {
        self.credit_type = Some(value);
        self
    }

    pub fn filtered(mut self, value: bool) -> Self {
        self.filtered = Some(value);
        self
    }

    pub fn ledger_id(mut self, value: impl Into<String>) -> Self {
        self.ledger_id = Some(value.into());
        self
    }

    pub fn metric_ids(mut self, value: Vec<String>) -> Self {
        self.metric_ids = Some(value);
        self
    }

    pub fn scope(mut self, value: CreditBalanceScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreditBalance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`available`](CreditBalanceBuilder::available)
    /// - [`credit_type`](CreditBalanceBuilder::credit_type)
    /// - [`filtered`](CreditBalanceBuilder::filtered)
    /// - [`metric_ids`](CreditBalanceBuilder::metric_ids)
    pub fn build(self) -> Result<CreditBalance, BuildError> {
        Ok(CreditBalance {
            available: self
                .available
                .ok_or_else(|| BuildError::missing_field("available"))?,
            credit_type: self
                .credit_type
                .ok_or_else(|| BuildError::missing_field("credit_type"))?,
            filtered: self
                .filtered
                .ok_or_else(|| BuildError::missing_field("filtered"))?,
            ledger_id: self.ledger_id,
            metric_ids: self
                .metric_ids
                .ok_or_else(|| BuildError::missing_field("metric_ids"))?,
            scope: self.scope,
        })
    }
}
