pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LineItemPayload {
    /// Unique identifier for the line item
    pub id: String,
    /// Invoice the line item belonged to
    #[serde(rename = "invoiceId")]
    pub invoice_id: String,
    /// Line item status at the time of the event
    pub status: String,
}

impl LineItemPayload {
    pub fn builder() -> LineItemPayloadBuilder {
        <LineItemPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LineItemPayloadBuilder {
    id: Option<String>,
    invoice_id: Option<String>,
    status: Option<String>,
}

impl LineItemPayloadBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LineItemPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LineItemPayloadBuilder::id)
    /// - [`invoice_id`](LineItemPayloadBuilder::invoice_id)
    /// - [`status`](LineItemPayloadBuilder::status)
    pub fn build(self) -> Result<LineItemPayload, BuildError> {
        Ok(LineItemPayload {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
