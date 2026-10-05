pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CustomerLookupResponse {
    /// Alguna customer ID the alias resolves to
    pub customer_id: String,
}

impl CustomerLookupResponse {
    pub fn builder() -> CustomerLookupResponseBuilder {
        <CustomerLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomerLookupResponseBuilder {
    customer_id: Option<String>,
}

impl CustomerLookupResponseBuilder {
    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CustomerLookupResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer_id`](CustomerLookupResponseBuilder::customer_id)
    pub fn build(self) -> Result<CustomerLookupResponse, BuildError> {
        Ok(CustomerLookupResponse {
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
        })
    }
}
