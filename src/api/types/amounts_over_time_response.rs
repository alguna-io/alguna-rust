pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AmountsOverTimeResponse {
    /// ISO 4217 currency code
    pub currency: String,
    /// Chronologically ordered data points
    pub points: Vec<DataPoint>,
    /// Time resolution of data points
    pub resolution: AmountsOverTimeResponseResolution,
}

impl AmountsOverTimeResponse {
    pub fn builder() -> AmountsOverTimeResponseBuilder {
        <AmountsOverTimeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AmountsOverTimeResponseBuilder {
    currency: Option<String>,
    points: Option<Vec<DataPoint>>,
    resolution: Option<AmountsOverTimeResponseResolution>,
}

impl AmountsOverTimeResponseBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn points(mut self, value: Vec<DataPoint>) -> Self {
        self.points = Some(value);
        self
    }

    pub fn resolution(mut self, value: AmountsOverTimeResponseResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AmountsOverTimeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](AmountsOverTimeResponseBuilder::currency)
    /// - [`points`](AmountsOverTimeResponseBuilder::points)
    /// - [`resolution`](AmountsOverTimeResponseBuilder::resolution)
    pub fn build(self) -> Result<AmountsOverTimeResponse, BuildError> {
        Ok(AmountsOverTimeResponse {
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
