pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ArAgingSummaryResponse {
    /// Mean unpaid balance per open invoice
    pub avg_balance_per_open_invoice: String,
    /// Invoice-weighted mean days past due, to one decimal place. Omitted when nothing is overdue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_days_overdue: Option<String>,
    /// Invoice-weighted mean days from issue to payment, to one decimal place. Omitted when there is no paid history.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_days_to_pay: Option<String>,
    /// Open balance split by days past due. The buckets sum to total_open_balance.
    pub buckets: ArAgingBuckets,
    /// Organization base currency that all amounts are converted to
    pub currency: String,
    /// Number of issued or overdue invoices
    pub open_invoice_count: i64,
    /// Past-due portion of total_open_balance. Equals the sum of every bucket except current.
    pub overdue_balance: String,
    /// Number of invoices past their due date
    pub overdue_invoice_count: i64,
    /// Total unpaid balance across all open invoices
    pub total_open_balance: String,
}

impl ArAgingSummaryResponse {
    pub fn builder() -> ArAgingSummaryResponseBuilder {
        <ArAgingSummaryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ArAgingSummaryResponseBuilder {
    avg_balance_per_open_invoice: Option<String>,
    avg_days_overdue: Option<String>,
    avg_days_to_pay: Option<String>,
    buckets: Option<ArAgingBuckets>,
    currency: Option<String>,
    open_invoice_count: Option<i64>,
    overdue_balance: Option<String>,
    overdue_invoice_count: Option<i64>,
    total_open_balance: Option<String>,
}

impl ArAgingSummaryResponseBuilder {
    pub fn avg_balance_per_open_invoice(mut self, value: impl Into<String>) -> Self {
        self.avg_balance_per_open_invoice = Some(value.into());
        self
    }

    pub fn avg_days_overdue(mut self, value: impl Into<String>) -> Self {
        self.avg_days_overdue = Some(value.into());
        self
    }

    pub fn avg_days_to_pay(mut self, value: impl Into<String>) -> Self {
        self.avg_days_to_pay = Some(value.into());
        self
    }

    pub fn buckets(mut self, value: ArAgingBuckets) -> Self {
        self.buckets = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn open_invoice_count(mut self, value: i64) -> Self {
        self.open_invoice_count = Some(value);
        self
    }

    pub fn overdue_balance(mut self, value: impl Into<String>) -> Self {
        self.overdue_balance = Some(value.into());
        self
    }

    pub fn overdue_invoice_count(mut self, value: i64) -> Self {
        self.overdue_invoice_count = Some(value);
        self
    }

    pub fn total_open_balance(mut self, value: impl Into<String>) -> Self {
        self.total_open_balance = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ArAgingSummaryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`avg_balance_per_open_invoice`](ArAgingSummaryResponseBuilder::avg_balance_per_open_invoice)
    /// - [`buckets`](ArAgingSummaryResponseBuilder::buckets)
    /// - [`currency`](ArAgingSummaryResponseBuilder::currency)
    /// - [`open_invoice_count`](ArAgingSummaryResponseBuilder::open_invoice_count)
    /// - [`overdue_balance`](ArAgingSummaryResponseBuilder::overdue_balance)
    /// - [`overdue_invoice_count`](ArAgingSummaryResponseBuilder::overdue_invoice_count)
    /// - [`total_open_balance`](ArAgingSummaryResponseBuilder::total_open_balance)
    pub fn build(self) -> Result<ArAgingSummaryResponse, BuildError> {
        Ok(ArAgingSummaryResponse {
            avg_balance_per_open_invoice: self
                .avg_balance_per_open_invoice
                .ok_or_else(|| BuildError::missing_field("avg_balance_per_open_invoice"))?,
            avg_days_overdue: self.avg_days_overdue,
            avg_days_to_pay: self.avg_days_to_pay,
            buckets: self
                .buckets
                .ok_or_else(|| BuildError::missing_field("buckets"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            open_invoice_count: self
                .open_invoice_count
                .ok_or_else(|| BuildError::missing_field("open_invoice_count"))?,
            overdue_balance: self
                .overdue_balance
                .ok_or_else(|| BuildError::missing_field("overdue_balance"))?,
            overdue_invoice_count: self
                .overdue_invoice_count
                .ok_or_else(|| BuildError::missing_field("overdue_invoice_count"))?,
            total_open_balance: self
                .total_open_balance
                .ok_or_else(|| BuildError::missing_field("total_open_balance"))?,
        })
    }
}
