pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QuotePhaseListResponse {
    /// The quote's phases in schedule order
    pub data: Vec<QuotePhaseResponse>,
}

impl QuotePhaseListResponse {
    pub fn builder() -> QuotePhaseListResponseBuilder {
        <QuotePhaseListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuotePhaseListResponseBuilder {
    data: Option<Vec<QuotePhaseResponse>>,
}

impl QuotePhaseListResponseBuilder {
    pub fn data(mut self, value: Vec<QuotePhaseResponse>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuotePhaseListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](QuotePhaseListResponseBuilder::data)
    pub fn build(self) -> Result<QuotePhaseListResponse, BuildError> {
        Ok(QuotePhaseListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
