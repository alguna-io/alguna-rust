pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PriceEscalationResponse {
    /// Whether price escalation is enabled
    pub enabled: bool,
    /// Whether to escalate metered unit rates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalate_metered_unit_rates: Option<bool>,
    /// Interval between escalations in months
    pub interval_months: i64,
    /// Escalation percentage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
    /// Escalation type
    pub r#type: String,
}

impl PriceEscalationResponse {
    pub fn builder() -> PriceEscalationResponseBuilder {
        <PriceEscalationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceEscalationResponseBuilder {
    enabled: Option<bool>,
    escalate_metered_unit_rates: Option<bool>,
    interval_months: Option<i64>,
    percentage: Option<String>,
    r#type: Option<String>,
}

impl PriceEscalationResponseBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn escalate_metered_unit_rates(mut self, value: bool) -> Self {
        self.escalate_metered_unit_rates = Some(value);
        self
    }

    pub fn interval_months(mut self, value: i64) -> Self {
        self.interval_months = Some(value);
        self
    }

    pub fn percentage(mut self, value: impl Into<String>) -> Self {
        self.percentage = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PriceEscalationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](PriceEscalationResponseBuilder::enabled)
    /// - [`interval_months`](PriceEscalationResponseBuilder::interval_months)
    /// - [`r#type`](PriceEscalationResponseBuilder::r#type)
    pub fn build(self) -> Result<PriceEscalationResponse, BuildError> {
        Ok(PriceEscalationResponse {
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
            escalate_metered_unit_rates: self.escalate_metered_unit_rates,
            interval_months: self
                .interval_months
                .ok_or_else(|| BuildError::missing_field("interval_months"))?,
            percentage: self.percentage,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
