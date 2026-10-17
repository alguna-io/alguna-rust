pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QuotePhaseItemListResponse {
    /// The phase's priced items
    pub data: Vec<SubscriptionPhaseItemResponse>,
}

impl QuotePhaseItemListResponse {
    pub fn builder() -> QuotePhaseItemListResponseBuilder {
        <QuotePhaseItemListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuotePhaseItemListResponseBuilder {
    data: Option<Vec<SubscriptionPhaseItemResponse>>,
}

impl QuotePhaseItemListResponseBuilder {
    pub fn data(mut self, value: Vec<SubscriptionPhaseItemResponse>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuotePhaseItemListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](QuotePhaseItemListResponseBuilder::data)
    pub fn build(self) -> Result<QuotePhaseItemListResponse, BuildError> {
        Ok(QuotePhaseItemListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
