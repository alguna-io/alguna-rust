pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RenewalInput {
    /// Whether the subscription auto-renews
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// Renewal duration in months
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_months: Option<i64>,
    /// Renewal period type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_type: Option<String>,
}

impl RenewalInput {
    pub fn builder() -> RenewalInputBuilder {
        <RenewalInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RenewalInputBuilder {
    auto_renew: Option<bool>,
    duration_months: Option<i64>,
    period_type: Option<String>,
}

impl RenewalInputBuilder {
    pub fn auto_renew(mut self, value: bool) -> Self {
        self.auto_renew = Some(value);
        self
    }

    pub fn duration_months(mut self, value: i64) -> Self {
        self.duration_months = Some(value);
        self
    }

    pub fn period_type(mut self, value: impl Into<String>) -> Self {
        self.period_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RenewalInput`].
    pub fn build(self) -> Result<RenewalInput, BuildError> {
        Ok(RenewalInput {
            auto_renew: self.auto_renew,
            duration_months: self.duration_months,
            period_type: self.period_type,
        })
    }
}
