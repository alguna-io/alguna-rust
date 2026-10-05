pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentsListQueryRequest {
    /// Filter by customer ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Filter by invoice ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Number of items to return per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of items to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl PaymentsListQueryRequest {
    pub fn builder() -> PaymentsListQueryRequestBuilder {
        <PaymentsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentsListQueryRequestBuilder {
    customer_id: Option<String>,
    invoice_id: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl PaymentsListQueryRequestBuilder {
    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentsListQueryRequest`].
    pub fn build(self) -> Result<PaymentsListQueryRequest, BuildError> {
        Ok(PaymentsListQueryRequest {
            customer_id: self.customer_id,
            invoice_id: self.invoice_id,
            limit: self.limit,
            offset: self.offset,
        })
    }
}
