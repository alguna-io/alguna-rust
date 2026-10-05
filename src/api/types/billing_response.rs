pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BillingResponse {
    /// Whether invoices are auto-issued
    pub auto_issue_invoices: bool,
    /// Whether invoices are auto-paid
    pub auto_pay_invoices: bool,
    /// First billing date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_billing_date: Option<DateTime<FixedOffset>>,
    /// How invoices are collected
    pub payment_collection_method: PaymentCollectionMethod,
    /// Saved payment method that auto-pay charges for this subscription. When absent, auto-pay charges the customer's default payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    /// Payment terms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<String>,
}

impl BillingResponse {
    pub fn builder() -> BillingResponseBuilder {
        <BillingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingResponseBuilder {
    auto_issue_invoices: Option<bool>,
    auto_pay_invoices: Option<bool>,
    first_billing_date: Option<DateTime<FixedOffset>>,
    payment_collection_method: Option<PaymentCollectionMethod>,
    payment_method_id: Option<String>,
    payment_terms: Option<String>,
}

impl BillingResponseBuilder {
    pub fn auto_issue_invoices(mut self, value: bool) -> Self {
        self.auto_issue_invoices = Some(value);
        self
    }

    pub fn auto_pay_invoices(mut self, value: bool) -> Self {
        self.auto_pay_invoices = Some(value);
        self
    }

    pub fn first_billing_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.first_billing_date = Some(value);
        self
    }

    pub fn payment_collection_method(mut self, value: PaymentCollectionMethod) -> Self {
        self.payment_collection_method = Some(value);
        self
    }

    pub fn payment_method_id(mut self, value: impl Into<String>) -> Self {
        self.payment_method_id = Some(value.into());
        self
    }

    pub fn payment_terms(mut self, value: impl Into<String>) -> Self {
        self.payment_terms = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BillingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auto_issue_invoices`](BillingResponseBuilder::auto_issue_invoices)
    /// - [`auto_pay_invoices`](BillingResponseBuilder::auto_pay_invoices)
    /// - [`payment_collection_method`](BillingResponseBuilder::payment_collection_method)
    pub fn build(self) -> Result<BillingResponse, BuildError> {
        Ok(BillingResponse {
            auto_issue_invoices: self
                .auto_issue_invoices
                .ok_or_else(|| BuildError::missing_field("auto_issue_invoices"))?,
            auto_pay_invoices: self
                .auto_pay_invoices
                .ok_or_else(|| BuildError::missing_field("auto_pay_invoices"))?,
            first_billing_date: self.first_billing_date,
            payment_collection_method: self
                .payment_collection_method
                .ok_or_else(|| BuildError::missing_field("payment_collection_method"))?,
            payment_method_id: self.payment_method_id,
            payment_terms: self.payment_terms,
        })
    }
}
