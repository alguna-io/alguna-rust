pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContractInput {
    /// Duration in months
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_months: Option<i64>,
    /// Contract period type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_type: Option<String>,
    /// Contract start date (YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
}

impl ContractInput {
    pub fn builder() -> ContractInputBuilder {
        <ContractInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContractInputBuilder {
    duration_months: Option<i64>,
    period_type: Option<String>,
    start_date: Option<NaiveDate>,
}

impl ContractInputBuilder {
    pub fn duration_months(mut self, value: i64) -> Self {
        self.duration_months = Some(value);
        self
    }

    pub fn period_type(mut self, value: impl Into<String>) -> Self {
        self.period_type = Some(value.into());
        self
    }

    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContractInput`].
    pub fn build(self) -> Result<ContractInput, BuildError> {
        Ok(ContractInput {
            duration_months: self.duration_months,
            period_type: self.period_type,
            start_date: self.start_date,
        })
    }
}
