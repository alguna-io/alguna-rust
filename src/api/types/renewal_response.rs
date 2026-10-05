pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RenewalResponse {
    /// Whether the subscription auto-renews
    pub auto_renew: bool,
    /// Renewal duration in months
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_months: Option<i64>,
    /// Renewal period type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_type: Option<RenewalResponsePeriodType>,
}

impl RenewalResponse {
    pub fn builder() -> RenewalResponseBuilder {
        <RenewalResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RenewalResponseBuilder {
    auto_renew: Option<bool>,
    duration_months: Option<i64>,
    period_type: Option<RenewalResponsePeriodType>,
}

impl RenewalResponseBuilder {
    pub fn auto_renew(mut self, value: bool) -> Self {
        self.auto_renew = Some(value);
        self
    }

    pub fn duration_months(mut self, value: i64) -> Self {
        self.duration_months = Some(value);
        self
    }

    pub fn period_type(mut self, value: RenewalResponsePeriodType) -> Self {
        self.period_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RenewalResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auto_renew`](RenewalResponseBuilder::auto_renew)
    pub fn build(self) -> Result<RenewalResponse, BuildError> {
        Ok(RenewalResponse {
            auto_renew: self
                .auto_renew
                .ok_or_else(|| BuildError::missing_field("auto_renew"))?,
            duration_months: self.duration_months,
            period_type: self.period_type,
        })
    }
}
