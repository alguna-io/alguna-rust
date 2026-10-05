pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateInvoiceRequest {
    /// Invoice description or memo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Purchase order number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order_number: Option<String>,
}

impl UpdateInvoiceRequest {
    pub fn builder() -> UpdateInvoiceRequestBuilder {
        <UpdateInvoiceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateInvoiceRequestBuilder {
    description: Option<String>,
    purchase_order_number: Option<String>,
}

impl UpdateInvoiceRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn purchase_order_number(mut self, value: impl Into<String>) -> Self {
        self.purchase_order_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateInvoiceRequest`].
    pub fn build(self) -> Result<UpdateInvoiceRequest, BuildError> {
        Ok(UpdateInvoiceRequest {
            description: self.description,
            purchase_order_number: self.purchase_order_number,
        })
    }
}
