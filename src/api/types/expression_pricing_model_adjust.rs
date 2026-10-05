pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExpressionPricingModelAdjust {
    /// Expression-based charges (replaces entirely if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges: Option<Vec<ExpressionChargeInput>>,
    /// Alias-to-metric bindings referenced in expressions (replaces entirely if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_bindings: Option<Vec<MetricBinding>>,
}

impl ExpressionPricingModelAdjust {
    pub fn builder() -> ExpressionPricingModelAdjustBuilder {
        <ExpressionPricingModelAdjustBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExpressionPricingModelAdjustBuilder {
    charges: Option<Vec<ExpressionChargeInput>>,
    metric_bindings: Option<Vec<MetricBinding>>,
}

impl ExpressionPricingModelAdjustBuilder {
    pub fn charges(mut self, value: Vec<ExpressionChargeInput>) -> Self {
        self.charges = Some(value);
        self
    }

    pub fn metric_bindings(mut self, value: Vec<MetricBinding>) -> Self {
        self.metric_bindings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExpressionPricingModelAdjust`].
    pub fn build(self) -> Result<ExpressionPricingModelAdjust, BuildError> {
        Ok(ExpressionPricingModelAdjust {
            charges: self.charges,
            metric_bindings: self.metric_bindings,
        })
    }
}
