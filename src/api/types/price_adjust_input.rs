pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PriceAdjustInput {
    /// Billing direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_direction: Option<BillingDirection>,
    /// Billing frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<BillingFrequency>,
    /// Billing interval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_interval: Option<BillingInterval>,
    /// Whether to charge on contract start
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_on_contract_start: Option<bool>,
    /// Price-level discount (replaces entirely)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<DiscountInput>,
    /// Display order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i64>,
    /// Expression pricing model adjustment (charges replaced entirely)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_pricing_model: Option<ExpressionPricingModelAdjust>,
    /// Fee type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_type: Option<PricingClass>,
    /// Fixed pricing model adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_pricing_model: Option<FixedPricingModelAdjust>,
    /// Graduated percentage pricing model adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduated_percentage_pricing_model: Option<GraduatedPercentagePricingModelAdjust>,
    /// Graduated tiered pricing model adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduated_tiered_pricing_model: Option<GraduatedTieredPricingModelAdjust>,
    /// Maximum spend per period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_spend: Option<SpendingThresholdInput>,
    /// Metric IDs (replaces entirely)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_ids: Option<Vec<String>>,
    /// Minimum spend per period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_spend: Option<SpendingThresholdInput>,
    /// Prepaid fixed tiered pricing model adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_fixed_tiered_pricing_model: Option<PrepaidFixedTieredPricingModelAdjust>,
    /// Prepaid tiered pricing model adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_tiered_pricing_model: Option<PrepaidTieredPricingModelAdjust>,
    /// Tiered percentage pricing model adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiered_percentage_pricing_model: Option<TieredPercentagePricingModelAdjust>,
    /// Tiered pricing model adjustment (tiers replaced entirely)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiered_pricing_model: Option<TieredPricingModelAdjust>,
    /// Trial period in days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    /// Unit pricing model adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_pricing_model: Option<UnitPricingModelAdjust>,
    /// Volume percentage pricing model adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_percentage_pricing_model: Option<VolumePercentagePricingModelAdjust>,
}

impl PriceAdjustInput {
    pub fn builder() -> PriceAdjustInputBuilder {
        <PriceAdjustInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceAdjustInputBuilder {
    billing_direction: Option<BillingDirection>,
    billing_frequency: Option<BillingFrequency>,
    billing_interval: Option<BillingInterval>,
    charge_on_contract_start: Option<bool>,
    discount: Option<DiscountInput>,
    display_order: Option<i64>,
    expression_pricing_model: Option<ExpressionPricingModelAdjust>,
    fee_type: Option<PricingClass>,
    fixed_pricing_model: Option<FixedPricingModelAdjust>,
    graduated_percentage_pricing_model: Option<GraduatedPercentagePricingModelAdjust>,
    graduated_tiered_pricing_model: Option<GraduatedTieredPricingModelAdjust>,
    maximum_spend: Option<SpendingThresholdInput>,
    metric_ids: Option<Vec<String>>,
    minimum_spend: Option<SpendingThresholdInput>,
    prepaid_fixed_tiered_pricing_model: Option<PrepaidFixedTieredPricingModelAdjust>,
    prepaid_tiered_pricing_model: Option<PrepaidTieredPricingModelAdjust>,
    tiered_percentage_pricing_model: Option<TieredPercentagePricingModelAdjust>,
    tiered_pricing_model: Option<TieredPricingModelAdjust>,
    trial_period_days: Option<i64>,
    unit_pricing_model: Option<UnitPricingModelAdjust>,
    volume_percentage_pricing_model: Option<VolumePercentagePricingModelAdjust>,
}

impl PriceAdjustInputBuilder {
    pub fn billing_direction(mut self, value: BillingDirection) -> Self {
        self.billing_direction = Some(value);
        self
    }

    pub fn billing_frequency(mut self, value: BillingFrequency) -> Self {
        self.billing_frequency = Some(value);
        self
    }

    pub fn billing_interval(mut self, value: BillingInterval) -> Self {
        self.billing_interval = Some(value);
        self
    }

    pub fn charge_on_contract_start(mut self, value: bool) -> Self {
        self.charge_on_contract_start = Some(value);
        self
    }

    pub fn discount(mut self, value: DiscountInput) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn display_order(mut self, value: i64) -> Self {
        self.display_order = Some(value);
        self
    }

    pub fn expression_pricing_model(mut self, value: ExpressionPricingModelAdjust) -> Self {
        self.expression_pricing_model = Some(value);
        self
    }

    pub fn fee_type(mut self, value: PricingClass) -> Self {
        self.fee_type = Some(value);
        self
    }

    pub fn fixed_pricing_model(mut self, value: FixedPricingModelAdjust) -> Self {
        self.fixed_pricing_model = Some(value);
        self
    }

    pub fn graduated_percentage_pricing_model(
        mut self,
        value: GraduatedPercentagePricingModelAdjust,
    ) -> Self {
        self.graduated_percentage_pricing_model = Some(value);
        self
    }

    pub fn graduated_tiered_pricing_model(
        mut self,
        value: GraduatedTieredPricingModelAdjust,
    ) -> Self {
        self.graduated_tiered_pricing_model = Some(value);
        self
    }

    pub fn maximum_spend(mut self, value: SpendingThresholdInput) -> Self {
        self.maximum_spend = Some(value);
        self
    }

    pub fn metric_ids(mut self, value: Vec<String>) -> Self {
        self.metric_ids = Some(value);
        self
    }

    pub fn minimum_spend(mut self, value: SpendingThresholdInput) -> Self {
        self.minimum_spend = Some(value);
        self
    }

    pub fn prepaid_fixed_tiered_pricing_model(
        mut self,
        value: PrepaidFixedTieredPricingModelAdjust,
    ) -> Self {
        self.prepaid_fixed_tiered_pricing_model = Some(value);
        self
    }

    pub fn prepaid_tiered_pricing_model(mut self, value: PrepaidTieredPricingModelAdjust) -> Self {
        self.prepaid_tiered_pricing_model = Some(value);
        self
    }

    pub fn tiered_percentage_pricing_model(
        mut self,
        value: TieredPercentagePricingModelAdjust,
    ) -> Self {
        self.tiered_percentage_pricing_model = Some(value);
        self
    }

    pub fn tiered_pricing_model(mut self, value: TieredPricingModelAdjust) -> Self {
        self.tiered_pricing_model = Some(value);
        self
    }

    pub fn trial_period_days(mut self, value: i64) -> Self {
        self.trial_period_days = Some(value);
        self
    }

    pub fn unit_pricing_model(mut self, value: UnitPricingModelAdjust) -> Self {
        self.unit_pricing_model = Some(value);
        self
    }

    pub fn volume_percentage_pricing_model(
        mut self,
        value: VolumePercentagePricingModelAdjust,
    ) -> Self {
        self.volume_percentage_pricing_model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PriceAdjustInput`].
    pub fn build(self) -> Result<PriceAdjustInput, BuildError> {
        Ok(PriceAdjustInput {
            billing_direction: self.billing_direction,
            billing_frequency: self.billing_frequency,
            billing_interval: self.billing_interval,
            charge_on_contract_start: self.charge_on_contract_start,
            discount: self.discount,
            display_order: self.display_order,
            expression_pricing_model: self.expression_pricing_model,
            fee_type: self.fee_type,
            fixed_pricing_model: self.fixed_pricing_model,
            graduated_percentage_pricing_model: self.graduated_percentage_pricing_model,
            graduated_tiered_pricing_model: self.graduated_tiered_pricing_model,
            maximum_spend: self.maximum_spend,
            metric_ids: self.metric_ids,
            minimum_spend: self.minimum_spend,
            prepaid_fixed_tiered_pricing_model: self.prepaid_fixed_tiered_pricing_model,
            prepaid_tiered_pricing_model: self.prepaid_tiered_pricing_model,
            tiered_percentage_pricing_model: self.tiered_percentage_pricing_model,
            tiered_pricing_model: self.tiered_pricing_model,
            trial_period_days: self.trial_period_days,
            unit_pricing_model: self.unit_pricing_model,
            volume_percentage_pricing_model: self.volume_percentage_pricing_model,
        })
    }
}
