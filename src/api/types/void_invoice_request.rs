pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoidInvoiceRequest {
    /// Date the invoice was voided; defaults to the current date. May be backdated but must not be in the future or before the invoice's issue date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub void_date: Option<DateTime<FixedOffset>>,
}

impl VoidInvoiceRequest {
    pub fn builder() -> VoidInvoiceRequestBuilder {
        <VoidInvoiceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoidInvoiceRequestBuilder {
    void_date: Option<DateTime<FixedOffset>>,
}

impl VoidInvoiceRequestBuilder {
    pub fn void_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.void_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoidInvoiceRequest`].
    pub fn build(self) -> Result<VoidInvoiceRequest, BuildError> {
        Ok(VoidInvoiceRequest {
            void_date: self.void_date,
        })
    }
}
