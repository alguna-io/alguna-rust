pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TieredPercentagePricingModelInput {
    /// Percentage-based pricing tiers
    pub tiers: Vec<PercentageTierInput>,
}

impl TieredPercentagePricingModelInput {
    pub fn builder() -> TieredPercentagePricingModelInputBuilder {
        <TieredPercentagePricingModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TieredPercentagePricingModelInputBuilder {
    tiers: Option<Vec<PercentageTierInput>>,
}

impl TieredPercentagePricingModelInputBuilder {
    pub fn tiers(mut self, value: Vec<PercentageTierInput>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TieredPercentagePricingModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](TieredPercentagePricingModelInputBuilder::tiers)
    pub fn build(self) -> Result<TieredPercentagePricingModelInput, BuildError> {
        Ok(TieredPercentagePricingModelInput {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
        })
    }
}
