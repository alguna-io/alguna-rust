pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefundsListQueryRequest {
    /// Filter by ISO 4217 currency code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Filter by customer ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Number of items to return per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of items to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Filter by payment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    /// Sort field and direction in format field:order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Filter by refund status. Pass a comma-separated list to match any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<ListRefundsRequestStatusItem>>,
}

impl RefundsListQueryRequest {
    pub fn builder() -> RefundsListQueryRequestBuilder {
        <RefundsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundsListQueryRequestBuilder {
    currency: Option<String>,
    customer_id: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
    payment_id: Option<String>,
    sort: Option<String>,
    status: Option<Vec<ListRefundsRequestStatusItem>>,
}

impl RefundsListQueryRequestBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
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

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn sort(mut self, value: impl Into<String>) -> Self {
        self.sort = Some(value.into());
        self
    }

    pub fn status(mut self, value: Vec<ListRefundsRequestStatusItem>) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundsListQueryRequest`].
    pub fn build(self) -> Result<RefundsListQueryRequest, BuildError> {
        Ok(RefundsListQueryRequest {
            currency: self.currency,
            customer_id: self.customer_id,
            limit: self.limit,
            offset: self.offset,
            payment_id: self.payment_id,
            sort: self.sort,
            status: self.status,
        })
    }
}
