pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QuoteWithdrawRequest {
    /// Optional reason recorded on the withdrawal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl QuoteWithdrawRequest {
    pub fn builder() -> QuoteWithdrawRequestBuilder {
        <QuoteWithdrawRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuoteWithdrawRequestBuilder {
    reason: Option<String>,
}

impl QuoteWithdrawRequestBuilder {
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QuoteWithdrawRequest`].
    pub fn build(self) -> Result<QuoteWithdrawRequest, BuildError> {
        Ok(QuoteWithdrawRequest {
            reason: self.reason,
        })
    }
}
