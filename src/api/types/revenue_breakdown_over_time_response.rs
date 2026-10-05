pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RevenueBreakdownOverTimeResponse {
    /// ISO 4217 currency code
    pub currency: String,
    /// Chronologically ordered data points
    pub points: Vec<RevenueBreakdownPoint>,
    /// Time resolution of data points
    pub resolution: RevenueBreakdownOverTimeResponseResolution,
}

impl RevenueBreakdownOverTimeResponse {
    pub fn builder() -> RevenueBreakdownOverTimeResponseBuilder {
        <RevenueBreakdownOverTimeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevenueBreakdownOverTimeResponseBuilder {
    currency: Option<String>,
    points: Option<Vec<RevenueBreakdownPoint>>,
    resolution: Option<RevenueBreakdownOverTimeResponseResolution>,
}

impl RevenueBreakdownOverTimeResponseBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn points(mut self, value: Vec<RevenueBreakdownPoint>) -> Self {
        self.points = Some(value);
        self
    }

    pub fn resolution(mut self, value: RevenueBreakdownOverTimeResponseResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevenueBreakdownOverTimeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](RevenueBreakdownOverTimeResponseBuilder::currency)
    /// - [`points`](RevenueBreakdownOverTimeResponseBuilder::points)
    /// - [`resolution`](RevenueBreakdownOverTimeResponseBuilder::resolution)
    pub fn build(self) -> Result<RevenueBreakdownOverTimeResponse, BuildError> {
        Ok(RevenueBreakdownOverTimeResponse {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            points: self
                .points
                .ok_or_else(|| BuildError::missing_field("points"))?,
            resolution: self
                .resolution
                .ok_or_else(|| BuildError::missing_field("resolution"))?,
        })
    }
}
