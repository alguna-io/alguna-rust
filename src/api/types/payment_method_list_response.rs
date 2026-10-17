pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListResponse {
    /// List of payment methods
    pub data: Vec<PaymentMethodResponse>,
}

impl PaymentMethodListResponse {
    pub fn builder() -> PaymentMethodListResponseBuilder {
        <PaymentMethodListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListResponseBuilder {
    data: Option<Vec<PaymentMethodResponse>>,
}

impl PaymentMethodListResponseBuilder {
    pub fn data(mut self, value: Vec<PaymentMethodResponse>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](PaymentMethodListResponseBuilder::data)
    pub fn build(self) -> Result<PaymentMethodListResponse, BuildError> {
        Ok(PaymentMethodListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
