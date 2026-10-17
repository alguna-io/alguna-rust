pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ArAgingCustomerEntry {
    /// Mean unpaid balance per open invoice
    pub avg_balance_per_open_invoice: String,
    /// Mean days past due across the overdue invoices, to one decimal place. Omitted when nothing is overdue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_days_overdue: Option<String>,
    /// Historical mean days from issue to payment, to one decimal place. Omitted when the customer has no paid history in this currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_days_to_pay: Option<String>,
    /// Open balance split by days past due. The buckets sum to total_open_balance.
    pub buckets: ArAgingBuckets,
    /// ISO 4217 currency code for this row
    pub currency: String,
    /// URL of the customer's icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_icon_url: Option<String>,
    /// Customer ID
    pub customer_id: String,
    /// Customer name
    pub customer_name: String,
    /// Number of issued or overdue invoices
    pub open_invoice_count: i64,
    /// Number of invoices past their due date
    pub overdue_invoice_count: i64,
    /// Total unpaid balance across the customer's open invoices in this currency
    pub total_open_balance: String,
}

impl ArAgingCustomerEntry {
    pub fn builder() -> ArAgingCustomerEntryBuilder {
        <ArAgingCustomerEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ArAgingCustomerEntryBuilder {
    avg_balance_per_open_invoice: Option<String>,
    avg_days_overdue: Option<String>,
    avg_days_to_pay: Option<String>,
    buckets: Option<ArAgingBuckets>,
    currency: Option<String>,
    customer_icon_url: Option<String>,
    customer_id: Option<String>,
    customer_name: Option<String>,
    open_invoice_count: Option<i64>,
    overdue_invoice_count: Option<i64>,
    total_open_balance: Option<String>,
}

impl ArAgingCustomerEntryBuilder {
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

    pub fn customer_icon_url(mut self, value: impl Into<String>) -> Self {
        self.customer_icon_url = Some(value.into());
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn customer_name(mut self, value: impl Into<String>) -> Self {
        self.customer_name = Some(value.into());
        self
    }

    pub fn open_invoice_count(mut self, value: i64) -> Self {
        self.open_invoice_count = Some(value);
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

    /// Consumes the builder and constructs a [`ArAgingCustomerEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`avg_balance_per_open_invoice`](ArAgingCustomerEntryBuilder::avg_balance_per_open_invoice)
    /// - [`buckets`](ArAgingCustomerEntryBuilder::buckets)
    /// - [`currency`](ArAgingCustomerEntryBuilder::currency)
    /// - [`customer_id`](ArAgingCustomerEntryBuilder::customer_id)
    /// - [`customer_name`](ArAgingCustomerEntryBuilder::customer_name)
    /// - [`open_invoice_count`](ArAgingCustomerEntryBuilder::open_invoice_count)
    /// - [`overdue_invoice_count`](ArAgingCustomerEntryBuilder::overdue_invoice_count)
    /// - [`total_open_balance`](ArAgingCustomerEntryBuilder::total_open_balance)
    pub fn build(self) -> Result<ArAgingCustomerEntry, BuildError> {
        Ok(ArAgingCustomerEntry {
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
            customer_icon_url: self.customer_icon_url,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            customer_name: self
                .customer_name
                .ok_or_else(|| BuildError::missing_field("customer_name"))?,
            open_invoice_count: self
                .open_invoice_count
                .ok_or_else(|| BuildError::missing_field("open_invoice_count"))?,
            overdue_invoice_count: self
                .overdue_invoice_count
                .ok_or_else(|| BuildError::missing_field("overdue_invoice_count"))?,
            total_open_balance: self
                .total_open_balance
                .ok_or_else(|| BuildError::missing_field("total_open_balance"))?,
        })
    }
}
