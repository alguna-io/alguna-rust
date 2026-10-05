pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DataPoint {
    /// Fixed (recurring) revenue amount
    pub amount_fixed: String,
    /// Metered (usage-based) revenue amount
    pub amount_metered: String,
    /// Revenue over-time only: portion of amount_fixed from one-off charges, already included in it. Always 0 on billings over-time, which has no one-off breakdown.
    pub amount_one_off_fixed: String,
    /// Revenue over-time only: portion of amount_metered from one-off charges, already included in it. Always 0 on billings over-time, which has no one-off breakdown.
    pub amount_one_off_metered: String,
    /// Start date of the period
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub date: DateTime<FixedOffset>,
}

impl DataPoint {
    pub fn builder() -> DataPointBuilder {
        <DataPointBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DataPointBuilder {
    amount_fixed: Option<String>,
    amount_metered: Option<String>,
    amount_one_off_fixed: Option<String>,
    amount_one_off_metered: Option<String>,
    date: Option<DateTime<FixedOffset>>,
}

impl DataPointBuilder {
    pub fn amount_fixed(mut self, value: impl Into<String>) -> Self {
        self.amount_fixed = Some(value.into());
        self
    }

    pub fn amount_metered(mut self, value: impl Into<String>) -> Self {
        self.amount_metered = Some(value.into());
        self
    }

    pub fn amount_one_off_fixed(mut self, value: impl Into<String>) -> Self {
        self.amount_one_off_fixed = Some(value.into());
        self
    }

    pub fn amount_one_off_metered(mut self, value: impl Into<String>) -> Self {
        self.amount_one_off_metered = Some(value.into());
        self
    }

    pub fn date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DataPoint`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_fixed`](DataPointBuilder::amount_fixed)
    /// - [`amount_metered`](DataPointBuilder::amount_metered)
    /// - [`amount_one_off_fixed`](DataPointBuilder::amount_one_off_fixed)
    /// - [`amount_one_off_metered`](DataPointBuilder::amount_one_off_metered)
    /// - [`date`](DataPointBuilder::date)
    pub fn build(self) -> Result<DataPoint, BuildError> {
        Ok(DataPoint {
            amount_fixed: self
                .amount_fixed
                .ok_or_else(|| BuildError::missing_field("amount_fixed"))?,
            amount_metered: self
                .amount_metered
                .ok_or_else(|| BuildError::missing_field("amount_metered"))?,
            amount_one_off_fixed: self
                .amount_one_off_fixed
                .ok_or_else(|| BuildError::missing_field("amount_one_off_fixed"))?,
            amount_one_off_metered: self
                .amount_one_off_metered
                .ok_or_else(|| BuildError::missing_field("amount_one_off_metered"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
        })
    }
}
