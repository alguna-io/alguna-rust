pub use crate::prelude::*;

/// Query parameters for getCreditBalance
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCreditBalanceQueryRequest {
    /// Point in time to evaluate the balance at (RFC3339). Defaults to now.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub as_of: Option<DateTime<FixedOffset>>,
}

impl GetCreditBalanceQueryRequest {
    pub fn builder() -> GetCreditBalanceQueryRequestBuilder {
        <GetCreditBalanceQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCreditBalanceQueryRequestBuilder {
    as_of: Option<DateTime<FixedOffset>>,
}

impl GetCreditBalanceQueryRequestBuilder {
    pub fn as_of(mut self, value: DateTime<FixedOffset>) -> Self {
        self.as_of = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCreditBalanceQueryRequest`].
    pub fn build(self) -> Result<GetCreditBalanceQueryRequest, BuildError> {
        Ok(GetCreditBalanceQueryRequest { as_of: self.as_of })
    }
}
