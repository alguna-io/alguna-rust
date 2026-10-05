pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QuotePhaseInput {
    /// Human-readable note on the phase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Complete set of priced items for this phase.
    pub items: Vec<VersionItemInput>,
    /// When the phase starts (YYYY-MM-DD). Alternative to start_month_offset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    /// Months after the billing start when the phase begins; resolved into a date at signature. Alternative to start_date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_month_offset: Option<i64>,
}

impl QuotePhaseInput {
    pub fn builder() -> QuotePhaseInputBuilder {
        <QuotePhaseInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuotePhaseInputBuilder {
    description: Option<String>,
    items: Option<Vec<VersionItemInput>>,
    start_date: Option<NaiveDate>,
    start_month_offset: Option<i64>,
}

impl QuotePhaseInputBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<VersionItemInput>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn start_month_offset(mut self, value: i64) -> Self {
        self.start_month_offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuotePhaseInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](QuotePhaseInputBuilder::items)
    pub fn build(self) -> Result<QuotePhaseInput, BuildError> {
        Ok(QuotePhaseInput {
            description: self.description,
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
            start_date: self.start_date,
            start_month_offset: self.start_month_offset,
        })
    }
}
