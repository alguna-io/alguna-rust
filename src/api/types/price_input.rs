pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PriceInput {
    /// Billing direction
    pub billing_direction: BillingDirection,
    /// Billing frequency (required for fixed fee type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<BillingFrequency>,
    /// Legacy named billing interval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_interval: Option<BillingInterval>,
    /// Positive count of BillingIntervalUnits per period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_interval_count: Option<i64>,
    /// Unit of the billing cadence (currently only 'month' accepted)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_interval_unit: Option<BillingIntervalUnit>,
    /// Whether to charge on contract start
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_on_contract_start: Option<bool>,
    /// Price-level discount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<DiscountInput>,
    /// Display order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i64>,
    /// Expression pricing model (required when type=expression)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_pricing_model: Option<ExpressionPricingModelInput>,
    /// Fee type
    pub fee_type: PricingClass,
    /// Fixed pricing model (required when type=fixed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_pricing_model: Option<FixedPricingModelInput>,
    /// Graduated percentage pricing model (required when type=graduated_percentage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduated_percentage_pricing_model: Option<GraduatedPercentagePricingModelInput>,
    /// Graduated tiered pricing model (required when type=graduated_tiered)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduated_tiered_pricing_model: Option<GraduatedTieredPricingModelInput>,
    /// Matrix pricing model (required when type=matrix)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix_pricing_model: Option<MatrixPricingModelInput>,
    /// Maximum spend per period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_spend: Option<SpendingThresholdInput>,
    /// Metric IDs (required for metered fee type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_ids: Option<Vec<String>>,
    /// Minimum spend per period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_spend: Option<SpendingThresholdInput>,
    /// Prepaid fixed tiered pricing model (required when type=prepaid_fixed_tiered)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_fixed_tiered_pricing_model: Option<PrepaidFixedTieredPricingModelInput>,
    /// Prepaid tiered pricing model (required when type=prepaid_tiered)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_tiered_pricing_model: Option<PrepaidTieredPricingModelInput>,
    /// Positive count of RecurringInstalmentIntervalUnits per instalment; must divide cleanly into the billing interval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_instalment_interval_count: Option<i64>,
    /// Optional sub-cadence unit splitting the advance charge into instalments within each billing cycle (currently only 'month' accepted)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_instalment_interval_unit: Option<BillingIntervalUnit>,
    /// Tiered percentage pricing model (required when type=tiered_percentage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiered_percentage_pricing_model: Option<TieredPercentagePricingModelInput>,
    /// Tiered pricing model (required when type=tiered)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiered_pricing_model: Option<TieredPricingModelInput>,
    /// Trial period in days (minimum 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    /// Pricing type
    pub r#type: PricingType,
    /// Unit pricing model (required when type=unit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_pricing_model: Option<UnitPricingModelInput>,
    /// Volume percentage pricing model (required when type=volume_percentage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_percentage_pricing_model: Option<VolumePercentagePricingModelInput>,
}

impl PriceInput {
    pub fn builder() -> PriceInputBuilder {
        <PriceInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceInputBuilder {
    billing_direction: Option<BillingDirection>,
    billing_frequency: Option<BillingFrequency>,
    billing_interval: Option<BillingInterval>,
    billing_interval_count: Option<i64>,
    billing_interval_unit: Option<BillingIntervalUnit>,
    charge_on_contract_start: Option<bool>,
    discount: Option<DiscountInput>,
    display_order: Option<i64>,
    expression_pricing_model: Option<ExpressionPricingModelInput>,
    fee_type: Option<PricingClass>,
    fixed_pricing_model: Option<FixedPricingModelInput>,
    graduated_percentage_pricing_model: Option<GraduatedPercentagePricingModelInput>,
    graduated_tiered_pricing_model: Option<GraduatedTieredPricingModelInput>,
    matrix_pricing_model: Option<MatrixPricingModelInput>,
    maximum_spend: Option<SpendingThresholdInput>,
    metric_ids: Option<Vec<String>>,
    minimum_spend: Option<SpendingThresholdInput>,
    prepaid_fixed_tiered_pricing_model: Option<PrepaidFixedTieredPricingModelInput>,
    prepaid_tiered_pricing_model: Option<PrepaidTieredPricingModelInput>,
    recurring_instalment_interval_count: Option<i64>,
    recurring_instalment_interval_unit: Option<BillingIntervalUnit>,
    tiered_percentage_pricing_model: Option<TieredPercentagePricingModelInput>,
    tiered_pricing_model: Option<TieredPricingModelInput>,
    trial_period_days: Option<i64>,
    r#type: Option<PricingType>,
    unit_pricing_model: Option<UnitPricingModelInput>,
    volume_percentage_pricing_model: Option<VolumePercentagePricingModelInput>,
}

impl PriceInputBuilder {
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

    pub fn billing_interval_count(mut self, value: i64) -> Self {
        self.billing_interval_count = Some(value);
        self
    }

    pub fn billing_interval_unit(mut self, value: BillingIntervalUnit) -> Self {
        self.billing_interval_unit = Some(value);
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

    pub fn expression_pricing_model(mut self, value: ExpressionPricingModelInput) -> Self {
        self.expression_pricing_model = Some(value);
        self
    }

    pub fn fee_type(mut self, value: PricingClass) -> Self {
        self.fee_type = Some(value);
        self
    }

    pub fn fixed_pricing_model(mut self, value: FixedPricingModelInput) -> Self {
        self.fixed_pricing_model = Some(value);
        self
    }

    pub fn graduated_percentage_pricing_model(
        mut self,
        value: GraduatedPercentagePricingModelInput,
    ) -> Self {
        self.graduated_percentage_pricing_model = Some(value);
        self
    }

    pub fn graduated_tiered_pricing_model(
        mut self,
        value: GraduatedTieredPricingModelInput,
    ) -> Self {
        self.graduated_tiered_pricing_model = Some(value);
        self
    }

    pub fn matrix_pricing_model(mut self, value: MatrixPricingModelInput) -> Self {
        self.matrix_pricing_model = Some(value);
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
        value: PrepaidFixedTieredPricingModelInput,
    ) -> Self {
        self.prepaid_fixed_tiered_pricing_model = Some(value);
        self
    }

    pub fn prepaid_tiered_pricing_model(mut self, value: PrepaidTieredPricingModelInput) -> Self {
        self.prepaid_tiered_pricing_model = Some(value);
        self
    }

    pub fn recurring_instalment_interval_count(mut self, value: i64) -> Self {
        self.recurring_instalment_interval_count = Some(value);
        self
    }

    pub fn recurring_instalment_interval_unit(mut self, value: BillingIntervalUnit) -> Self {
        self.recurring_instalment_interval_unit = Some(value);
        self
    }

    pub fn tiered_percentage_pricing_model(
        mut self,
        value: TieredPercentagePricingModelInput,
    ) -> Self {
        self.tiered_percentage_pricing_model = Some(value);
        self
    }

    pub fn tiered_pricing_model(mut self, value: TieredPricingModelInput) -> Self {
        self.tiered_pricing_model = Some(value);
        self
    }

    pub fn trial_period_days(mut self, value: i64) -> Self {
        self.trial_period_days = Some(value);
        self
    }

    pub fn r#type(mut self, value: PricingType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn unit_pricing_model(mut self, value: UnitPricingModelInput) -> Self {
        self.unit_pricing_model = Some(value);
        self
    }

    pub fn volume_percentage_pricing_model(
        mut self,
        value: VolumePercentagePricingModelInput,
    ) -> Self {
        self.volume_percentage_pricing_model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PriceInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`billing_direction`](PriceInputBuilder::billing_direction)
    /// - [`fee_type`](PriceInputBuilder::fee_type)
    /// - [`r#type`](PriceInputBuilder::r#type)
    pub fn build(self) -> Result<PriceInput, BuildError> {
        Ok(PriceInput {
            billing_direction: self
                .billing_direction
                .ok_or_else(|| BuildError::missing_field("billing_direction"))?,
            billing_frequency: self.billing_frequency,
            billing_interval: self.billing_interval,
            billing_interval_count: self.billing_interval_count,
            billing_interval_unit: self.billing_interval_unit,
            charge_on_contract_start: self.charge_on_contract_start,
            discount: self.discount,
            display_order: self.display_order,
            expression_pricing_model: self.expression_pricing_model,
            fee_type: self
                .fee_type
                .ok_or_else(|| BuildError::missing_field("fee_type"))?,
            fixed_pricing_model: self.fixed_pricing_model,
            graduated_percentage_pricing_model: self.graduated_percentage_pricing_model,
            graduated_tiered_pricing_model: self.graduated_tiered_pricing_model,
            matrix_pricing_model: self.matrix_pricing_model,
            maximum_spend: self.maximum_spend,
            metric_ids: self.metric_ids,
            minimum_spend: self.minimum_spend,
            prepaid_fixed_tiered_pricing_model: self.prepaid_fixed_tiered_pricing_model,
            prepaid_tiered_pricing_model: self.prepaid_tiered_pricing_model,
            recurring_instalment_interval_count: self.recurring_instalment_interval_count,
            recurring_instalment_interval_unit: self.recurring_instalment_interval_unit,
            tiered_percentage_pricing_model: self.tiered_percentage_pricing_model,
            tiered_pricing_model: self.tiered_pricing_model,
            trial_period_days: self.trial_period_days,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            unit_pricing_model: self.unit_pricing_model,
            volume_percentage_pricing_model: self.volume_percentage_pricing_model,
        })
    }
}
