pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentAllocationPayload {
    /// Amount applied to this invoice, formatted to the currency's precision
    pub amount: String,
    /// Invoice this portion was applied to
    #[serde(rename = "invoiceId")]
    pub invoice_id: String,
}

impl PaymentAllocationPayload {
    pub fn builder() -> PaymentAllocationPayloadBuilder {
        <PaymentAllocationPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentAllocationPayloadBuilder {
    amount: Option<String>,
    invoice_id: Option<String>,
}

impl PaymentAllocationPayloadBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentAllocationPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](PaymentAllocationPayloadBuilder::amount)
    /// - [`invoice_id`](PaymentAllocationPayloadBuilder::invoice_id)
    pub fn build(self) -> Result<PaymentAllocationPayload, BuildError> {
        Ok(PaymentAllocationPayload {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
        })
    }
}
