pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateBillingInput {
    /// Whether to auto-issue invoices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_issue_invoices: Option<bool>,
    /// Whether to auto-pay invoices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay_invoices: Option<bool>,
    /// First billing date (YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_billing_date: Option<String>,
    /// How invoices are collected. manual: no automatic charge, and only allowed while auto_pay_invoices is false. autopay: Alguna charges the subscription's payment method, or the customer's default when none is set. external: payment is collected outside Alguna and Alguna never charges. When omitted, the current method is kept, except that turning auto_pay_invoices on changes manual to autopay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_collection_method: Option<PaymentCollectionMethod>,
    /// Saved payment method that auto-pay charges for this subscription instead of the customer's default. It must be an active payment method of the same customer, as listed by GET /customers/{id}/payment-methods. Not allowed with the external collection method. Send an empty string to remove it, so that auto-pay charges the customer's default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    /// Payment terms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<String>,
}

impl UpdateBillingInput {
    pub fn builder() -> UpdateBillingInputBuilder {
        <UpdateBillingInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBillingInputBuilder {
    auto_issue_invoices: Option<bool>,
    auto_pay_invoices: Option<bool>,
    first_billing_date: Option<String>,
    payment_collection_method: Option<PaymentCollectionMethod>,
    payment_method_id: Option<String>,
    payment_terms: Option<String>,
}

impl UpdateBillingInputBuilder {
    pub fn auto_issue_invoices(mut self, value: bool) -> Self {
        self.auto_issue_invoices = Some(value);
        self
    }

    pub fn auto_pay_invoices(mut self, value: bool) -> Self {
        self.auto_pay_invoices = Some(value);
        self
    }

    pub fn first_billing_date(mut self, value: impl Into<String>) -> Self {
        self.first_billing_date = Some(value.into());
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

    /// Consumes the builder and constructs a [`UpdateBillingInput`].
    pub fn build(self) -> Result<UpdateBillingInput, BuildError> {
        Ok(UpdateBillingInput {
            auto_issue_invoices: self.auto_issue_invoices,
            auto_pay_invoices: self.auto_pay_invoices,
            first_billing_date: self.first_billing_date,
            payment_collection_method: self.payment_collection_method,
            payment_method_id: self.payment_method_id,
            payment_terms: self.payment_terms,
        })
    }
}
