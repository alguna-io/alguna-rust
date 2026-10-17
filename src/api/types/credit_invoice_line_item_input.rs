pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreditInvoiceLineItemInput {
    /// Invoice line item to credit
    pub invoice_line_item_id: String,
    /// Units to credit, priced and taxed exactly as they were billed
    pub quantity: String,
}

impl CreditInvoiceLineItemInput {
    pub fn builder() -> CreditInvoiceLineItemInputBuilder {
        <CreditInvoiceLineItemInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditInvoiceLineItemInputBuilder {
    invoice_line_item_id: Option<String>,
    quantity: Option<String>,
}

impl CreditInvoiceLineItemInputBuilder {
    pub fn invoice_line_item_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_line_item_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreditInvoiceLineItemInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_line_item_id`](CreditInvoiceLineItemInputBuilder::invoice_line_item_id)
    /// - [`quantity`](CreditInvoiceLineItemInputBuilder::quantity)
    pub fn build(self) -> Result<CreditInvoiceLineItemInput, BuildError> {
        Ok(CreditInvoiceLineItemInput {
            invoice_line_item_id: self
                .invoice_line_item_id
                .ok_or_else(|| BuildError::missing_field("invoice_line_item_id"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}
