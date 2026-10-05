pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreditNotesListQueryRequest {
    /// Customer ID to list credit notes for
    pub customer_id: String,
    /// Number of items to return per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of items to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl CreditNotesListQueryRequest {
    pub fn builder() -> CreditNotesListQueryRequestBuilder {
        <CreditNotesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditNotesListQueryRequestBuilder {
    customer_id: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl CreditNotesListQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`CreditNotesListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer_id`](CreditNotesListQueryRequestBuilder::customer_id)
    pub fn build(self) -> Result<CreditNotesListQueryRequest, BuildError> {
        Ok(CreditNotesListQueryRequest {
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            limit: self.limit,
            offset: self.offset,
        })
    }
}
