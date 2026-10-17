pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QuotePhaseResponse {
    /// Annual contract value
    pub acv: String,
    /// Annual recurring revenue
    pub arr: String,
    /// ISO 4217 currency code
    pub currency: String,
    /// Note on the phase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When the phase ends
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<FixedOffset>>,
    /// Phase ID. This is the subscription version the phase becomes: after signature the same row is readable at /subscriptions/{subscription_id}/versions/{id}.
    pub id: String,
    /// Monthly recurring revenue
    pub mrr: String,
    /// Plan the phase was seeded from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// Quote ID
    pub quote_id: String,
    /// Position in the quote's schedule, from 1. Positional, not an identifier: deleting a phase renumbers the rest, so address a phase by id.
    pub sequence: i64,
    /// When the phase starts; null for an undated ramp phase until signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<FixedOffset>>,
    /// Months after the billing start when an undated phase begins
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_month_offset: Option<i64>,
    /// Total contract value of the phase over its term; null when no term is knowable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcv: Option<String>,
}

impl QuotePhaseResponse {
    pub fn builder() -> QuotePhaseResponseBuilder {
        <QuotePhaseResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuotePhaseResponseBuilder {
    acv: Option<String>,
    arr: Option<String>,
    currency: Option<String>,
    description: Option<String>,
    end_date: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    mrr: Option<String>,
    plan_id: Option<String>,
    quote_id: Option<String>,
    sequence: Option<i64>,
    start_date: Option<DateTime<FixedOffset>>,
    start_month_offset: Option<i64>,
    tcv: Option<String>,
}

impl QuotePhaseResponseBuilder {
    pub fn acv(mut self, value: impl Into<String>) -> Self {
        self.acv = Some(value.into());
        self
    }

    pub fn arr(mut self, value: impl Into<String>) -> Self {
        self.arr = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn mrr(mut self, value: impl Into<String>) -> Self {
        self.mrr = Some(value.into());
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn quote_id(mut self, value: impl Into<String>) -> Self {
        self.quote_id = Some(value.into());
        self
    }

    pub fn sequence(mut self, value: i64) -> Self {
        self.sequence = Some(value);
        self
    }

    pub fn start_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn start_month_offset(mut self, value: i64) -> Self {
        self.start_month_offset = Some(value);
        self
    }

    pub fn tcv(mut self, value: impl Into<String>) -> Self {
        self.tcv = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QuotePhaseResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`acv`](QuotePhaseResponseBuilder::acv)
    /// - [`arr`](QuotePhaseResponseBuilder::arr)
    /// - [`currency`](QuotePhaseResponseBuilder::currency)
    /// - [`id`](QuotePhaseResponseBuilder::id)
    /// - [`mrr`](QuotePhaseResponseBuilder::mrr)
    /// - [`quote_id`](QuotePhaseResponseBuilder::quote_id)
    /// - [`sequence`](QuotePhaseResponseBuilder::sequence)
    pub fn build(self) -> Result<QuotePhaseResponse, BuildError> {
        Ok(QuotePhaseResponse {
            acv: self.acv.ok_or_else(|| BuildError::missing_field("acv"))?,
            arr: self.arr.ok_or_else(|| BuildError::missing_field("arr"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            description: self.description,
            end_date: self.end_date,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            mrr: self.mrr.ok_or_else(|| BuildError::missing_field("mrr"))?,
            plan_id: self.plan_id,
            quote_id: self
                .quote_id
                .ok_or_else(|| BuildError::missing_field("quote_id"))?,
            sequence: self
                .sequence
                .ok_or_else(|| BuildError::missing_field("sequence"))?,
            start_date: self.start_date,
            start_month_offset: self.start_month_offset,
            tcv: self.tcv,
        })
    }
}
