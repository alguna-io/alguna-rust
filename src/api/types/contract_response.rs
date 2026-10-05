pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ContractResponse {
    /// Contract duration in months
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_months: Option<i64>,
    /// Contract end date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<FixedOffset>>,
    /// Contract period type
    pub period_type: ContractResponsePeriodType,
    /// Contract start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<FixedOffset>>,
}

impl ContractResponse {
    pub fn builder() -> ContractResponseBuilder {
        <ContractResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContractResponseBuilder {
    duration_months: Option<i64>,
    end_date: Option<DateTime<FixedOffset>>,
    period_type: Option<ContractResponsePeriodType>,
    start_date: Option<DateTime<FixedOffset>>,
}

impl ContractResponseBuilder {
    pub fn duration_months(mut self, value: i64) -> Self {
        self.duration_months = Some(value);
        self
    }

    pub fn end_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn period_type(mut self, value: ContractResponsePeriodType) -> Self {
        self.period_type = Some(value);
        self
    }

    pub fn start_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContractResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period_type`](ContractResponseBuilder::period_type)
    pub fn build(self) -> Result<ContractResponse, BuildError> {
        Ok(ContractResponse {
            duration_months: self.duration_months,
            end_date: self.end_date,
            period_type: self
                .period_type
                .ok_or_else(|| BuildError::missing_field("period_type"))?,
            start_date: self.start_date,
        })
    }
}
