pub use crate::prelude::*;

/// Query parameters for arAgingByCustomer
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ArAgingByCustomerQueryRequest {
    /// Filter by an alternative identifier (external ID) for the customer. Used when customer_id is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_alias: Option<String>,
    /// Filter by customer ID. Provide at most one of customer_id or customer_alias; omit both to report on every customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
}

impl ArAgingByCustomerQueryRequest {
    pub fn builder() -> ArAgingByCustomerQueryRequestBuilder {
        <ArAgingByCustomerQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ArAgingByCustomerQueryRequestBuilder {
    customer_alias: Option<String>,
    customer_id: Option<String>,
}

impl ArAgingByCustomerQueryRequestBuilder {
    pub fn customer_alias(mut self, value: impl Into<String>) -> Self {
        self.customer_alias = Some(value.into());
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ArAgingByCustomerQueryRequest`].
    pub fn build(self) -> Result<ArAgingByCustomerQueryRequest, BuildError> {
        Ok(ArAgingByCustomerQueryRequest {
            customer_alias: self.customer_alias,
            customer_id: self.customer_id,
        })
    }
}
