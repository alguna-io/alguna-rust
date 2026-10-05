pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCreditNoteLineItemInput {
    /// Line item description. Required unless the line credits an invoice line item, whose description it then takes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Invoice line item this line credits. Required when no unit_price is given.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item_id: Option<String>,
    /// Associated product ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Quantity. With unit_price it is the quantity being priced; without one it is the number of units credited off invoice_line_item_id.
    pub quantity: String,
    /// Tax-exclusive unit price. Tax is calculated on top of it. Omit it to credit invoice_line_item_id instead, which reuses the price and tax already charged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<String>,
}

impl CreateCreditNoteLineItemInput {
    pub fn builder() -> CreateCreditNoteLineItemInputBuilder {
        <CreateCreditNoteLineItemInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCreditNoteLineItemInputBuilder {
    description: Option<String>,
    invoice_line_item_id: Option<String>,
    product_id: Option<String>,
    quantity: Option<String>,
    unit_price: Option<String>,
}

impl CreateCreditNoteLineItemInputBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn invoice_line_item_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_line_item_id = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn unit_price(mut self, value: impl Into<String>) -> Self {
        self.unit_price = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateCreditNoteLineItemInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`quantity`](CreateCreditNoteLineItemInputBuilder::quantity)
    pub fn build(self) -> Result<CreateCreditNoteLineItemInput, BuildError> {
        Ok(CreateCreditNoteLineItemInput {
            description: self.description,
            invoice_line_item_id: self.invoice_line_item_id,
            product_id: self.product_id,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_price: self.unit_price,
        })
    }
}
