pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QuotesListQueryRequest {
    /// Filter by quote kind
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ListQuotesRequestKind>,
    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of quotes to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Filter by document status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListQuotesRequestStatus>,
    /// Only quotes documenting this subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}

impl QuotesListQueryRequest {
    pub fn builder() -> QuotesListQueryRequestBuilder {
        <QuotesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuotesListQueryRequestBuilder {
    kind: Option<ListQuotesRequestKind>,
    limit: Option<i64>,
    offset: Option<i64>,
    status: Option<ListQuotesRequestStatus>,
    subscription_id: Option<String>,
}

impl QuotesListQueryRequestBuilder {
    pub fn kind(mut self, value: ListQuotesRequestKind) -> Self {
        self.kind = Some(value);
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

    pub fn status(mut self, value: ListQuotesRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QuotesListQueryRequest`].
    pub fn build(self) -> Result<QuotesListQueryRequest, BuildError> {
        Ok(QuotesListQueryRequest {
            kind: self.kind,
            limit: self.limit,
            offset: self.offset,
            status: self.status,
            subscription_id: self.subscription_id,
        })
    }
}
