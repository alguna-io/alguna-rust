pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateRefundRequest {
    /// Refund amount
    pub amount: String,
    /// The payment to refund
    pub payment_id: String,
    /// Reason for the refund
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Text that appears on the customer's bank statement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

impl CreateRefundRequest {
    pub fn builder() -> CreateRefundRequestBuilder {
        <CreateRefundRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRefundRequestBuilder {
    amount: Option<String>,
    payment_id: Option<String>,
    reason: Option<String>,
    statement_descriptor: Option<String>,
}

impl CreateRefundRequestBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn statement_descriptor(mut self, value: impl Into<String>) -> Self {
        self.statement_descriptor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateRefundRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CreateRefundRequestBuilder::amount)
    /// - [`payment_id`](CreateRefundRequestBuilder::payment_id)
    pub fn build(self) -> Result<CreateRefundRequest, BuildError> {
        Ok(CreateRefundRequest {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            payment_id: self
                .payment_id
                .ok_or_else(|| BuildError::missing_field("payment_id"))?,
            reason: self.reason,
            statement_descriptor: self.statement_descriptor,
        })
    }
}
