pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentAllocation {
    /// Amount of the payment applied to the invoice
    pub amount: String,
    /// Invoice this portion of the payment was applied to
    pub invoice_id: String,
    /// Whether this allocation was reversed (invoice reopened); the amount is now unallocated
    pub reversed: bool,
    /// Allocation status
    pub status: PaymentAllocationStatus,
}

impl PaymentAllocation {
    pub fn builder() -> PaymentAllocationBuilder {
        <PaymentAllocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentAllocationBuilder {
    amount: Option<String>,
    invoice_id: Option<String>,
    reversed: Option<bool>,
    status: Option<PaymentAllocationStatus>,
}

impl PaymentAllocationBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn reversed(mut self, value: bool) -> Self {
        self.reversed = Some(value);
        self
    }

    pub fn status(mut self, value: PaymentAllocationStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentAllocation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](PaymentAllocationBuilder::amount)
    /// - [`invoice_id`](PaymentAllocationBuilder::invoice_id)
    /// - [`reversed`](PaymentAllocationBuilder::reversed)
    /// - [`status`](PaymentAllocationBuilder::status)
    pub fn build(self) -> Result<PaymentAllocation, BuildError> {
        Ok(PaymentAllocation {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            reversed: self
                .reversed
                .ok_or_else(|| BuildError::missing_field("reversed"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
