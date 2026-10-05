pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QuoteContractInput {
    /// Contract start date (YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
}

impl QuoteContractInput {
    pub fn builder() -> QuoteContractInputBuilder {
        <QuoteContractInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuoteContractInputBuilder {
    start_date: Option<NaiveDate>,
}

impl QuoteContractInputBuilder {
    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuoteContractInput`].
    pub fn build(self) -> Result<QuoteContractInput, BuildError> {
        Ok(QuoteContractInput {
            start_date: self.start_date,
        })
    }
}
