pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplyCreditNoteRequest {
    /// Whether to issue a refund when applying to a paid invoice
    pub do_refund: bool,
    /// Where to send the refund when refunding a paid invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_destination: Option<ApplyCreditNoteRequestRefundDestination>,
    /// Wallet to refund to when refund_destination is wallet for a paid-invoice refund
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
}

impl ApplyCreditNoteRequest {
    pub fn builder() -> ApplyCreditNoteRequestBuilder {
        <ApplyCreditNoteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplyCreditNoteRequestBuilder {
    do_refund: Option<bool>,
    refund_destination: Option<ApplyCreditNoteRequestRefundDestination>,
    wallet_id: Option<String>,
}

impl ApplyCreditNoteRequestBuilder {
    pub fn do_refund(mut self, value: bool) -> Self {
        self.do_refund = Some(value);
        self
    }

    pub fn refund_destination(mut self, value: ApplyCreditNoteRequestRefundDestination) -> Self {
        self.refund_destination = Some(value);
        self
    }

    pub fn wallet_id(mut self, value: impl Into<String>) -> Self {
        self.wallet_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApplyCreditNoteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`do_refund`](ApplyCreditNoteRequestBuilder::do_refund)
    pub fn build(self) -> Result<ApplyCreditNoteRequest, BuildError> {
        Ok(ApplyCreditNoteRequest {
            do_refund: self
                .do_refund
                .ok_or_else(|| BuildError::missing_field("do_refund"))?,
            refund_destination: self.refund_destination,
            wallet_id: self.wallet_id,
        })
    }
}
