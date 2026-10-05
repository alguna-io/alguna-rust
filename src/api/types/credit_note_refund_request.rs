pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreditNoteRefundRequest {
    /// Where the funds go. original_payment returns them to the payments that settled the invoice.
    pub destination: String,
}

impl CreditNoteRefundRequest {
    pub fn builder() -> CreditNoteRefundRequestBuilder {
        <CreditNoteRefundRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditNoteRefundRequestBuilder {
    destination: Option<String>,
}

impl CreditNoteRefundRequestBuilder {
    pub fn destination(mut self, value: impl Into<String>) -> Self {
        self.destination = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreditNoteRefundRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination`](CreditNoteRefundRequestBuilder::destination)
    pub fn build(self) -> Result<CreditNoteRefundRequest, BuildError> {
        Ok(CreditNoteRefundRequest {
            destination: self
                .destination
                .ok_or_else(|| BuildError::missing_field("destination"))?,
        })
    }
}
