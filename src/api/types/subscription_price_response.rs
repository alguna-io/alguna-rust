pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscriptionPriceResponse {
    /// Billing direction
    pub billing_direction: BillingDirection,
    /// Billing frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<BillingFrequency>,
    /// Billing interval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_interval: Option<BillingInterval>,
    /// Positive count of BillingIntervalUnits per period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_interval_count: Option<i64>,
    /// Unit of the billing cadence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_interval_unit: Option<BillingIntervalUnit>,
    /// Whether to charge on contract start
    pub charge_on_contract_start: bool,
    /// ISO 4217 currency code
    pub currency: String,
    /// Price-level discount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<DiscountResponse>,
    /// Display order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i64>,
    /// Expression pricing model (when type=expression)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_pricing_model: Option<ExpressionPricingModelResponse>,
    /// Fee type
    pub fee_type: PricingClass,
    /// Fixed pricing model (when type=fixed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_pricing_model: Option<FixedPricingModelResponse>,
    /// Graduated percentage pricing model (when type=graduated_percentage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduated_percentage_pricing_model: Option<GraduatedPercentagePricingModelResponse>,
    /// Graduated tiered pricing model (when type=graduated_tiered)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduated_tiered_pricing_model: Option<GraduatedTieredPricingModelResponse>,
    /// Matrix pricing model (when type=matrix)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix_pricing_model: Option<MatrixPricingModelResponse>,
    /// Maximum spend per period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_spend: Option<SpendingThresholdResponse>,
    /// Metric IDs for metered pricing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_ids: Option<Vec<String>>,
    /// Minimum spend per period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_spend: Option<SpendingThresholdResponse>,
    /// Prepaid fixed tiered pricing model (when type=prepaid_fixed_tiered)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_fixed_tiered_pricing_model: Option<PrepaidFixedTieredPricingModelResponse>,
    /// Prepaid tiered pricing model (when type=prepaid_tiered)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_tiered_pricing_model: Option<PrepaidTieredPricingModelResponse>,
    /// Tiered percentage pricing model (when type=tiered_percentage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiered_percentage_pricing_model: Option<TieredPercentagePricingModelResponse>,
    /// Tiered pricing model (when type=tiered)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiered_pricing_model: Option<TieredPricingModelResponse>,
    /// Trial period in days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    /// Pricing type
    pub r#type: PricingType,
    /// Unit pricing model (when type=unit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_pricing_model: Option<UnitPricingModelResponse>,
    /// Volume percentage pricing model (when type=volume_percentage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_percentage_pricing_model: Option<VolumePercentagePricingModelResponse>,
}

impl SubscriptionPriceResponse {
    pub fn builder() -> SubscriptionPriceResponseBuilder {
        <SubscriptionPriceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionPriceResponseBuilder {
    billing_direction: Option<BillingDirection>,
    billing_frequency: Option<BillingFrequency>,
    billing_interval: Option<BillingInterval>,
    billing_interval_count: Option<i64>,
    billing_interval_unit: Option<BillingIntervalUnit>,
    charge_on_contract_start: Option<bool>,
    currency: Option<String>,
    discount: Option<DiscountResponse>,
    display_order: Option<i64>,
    expression_pricing_model: Option<ExpressionPricingModelResponse>,
    fee_type: Option<PricingClass>,
    fixed_pricing_model: Option<FixedPricingModelResponse>,
    graduated_percentage_pricing_model: Option<GraduatedPercentagePricingModelResponse>,
    graduated_tiered_pricing_model: Option<GraduatedTieredPricingModelResponse>,
    matrix_pricing_model: Option<MatrixPricingModelResponse>,
    maximum_spend: Option<SpendingThresholdResponse>,
    metric_ids: Option<Vec<String>>,
    minimum_spend: Option<SpendingThresholdResponse>,
    prepaid_fixed_tiered_pricing_model: Option<PrepaidFixedTieredPricingModelResponse>,
    prepaid_tiered_pricing_model: Option<PrepaidTieredPricingModelResponse>,
    tiered_percentage_pricing_model: Option<TieredPercentagePricingModelResponse>,
    tiered_pricing_model: Option<TieredPricingModelResponse>,
    trial_period_days: Option<i64>,
    r#type: Option<PricingType>,
    unit_pricing_model: Option<UnitPricingModelResponse>,
    volume_percentage_pricing_model: Option<VolumePercentagePricingModelResponse>,
}

impl SubscriptionPriceResponseBuilder {
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

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn discount(mut self, value: DiscountResponse) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn display_order(mut self, value: i64) -> Self {
        self.display_order = Some(value);
        self
    }

    pub fn expression_pricing_model(mut self, value: ExpressionPricingModelResponse) -> Self {
        self.expression_pricing_model = Some(value);
        self
    }

    pub fn fee_type(mut self, value: PricingClass) -> Self {
        self.fee_type = Some(value);
        self
    }

    pub fn fixed_pricing_model(mut self, value: FixedPricingModelResponse) -> Self {
        self.fixed_pricing_model = Some(value);
        self
    }

    pub fn graduated_percentage_pricing_model(
        mut self,
        value: GraduatedPercentagePricingModelResponse,
    ) -> Self {
        self.graduated_percentage_pricing_model = Some(value);
        self
    }

    pub fn graduated_tiered_pricing_model(
        mut self,
        value: GraduatedTieredPricingModelResponse,
    ) -> Self {
        self.graduated_tiered_pricing_model = Some(value);
        self
    }

    pub fn matrix_pricing_model(mut self, value: MatrixPricingModelResponse) -> Self {
        self.matrix_pricing_model = Some(value);
        self
    }

    pub fn maximum_spend(mut self, value: SpendingThresholdResponse) -> Self {
        self.maximum_spend = Some(value);
        self
    }

    pub fn metric_ids(mut self, value: Vec<String>) -> Self {
        self.metric_ids = Some(value);
        self
    }

    pub fn minimum_spend(mut self, value: SpendingThresholdResponse) -> Self {
        self.minimum_spend = Some(value);
        self
    }

    pub fn prepaid_fixed_tiered_pricing_model(
        mut self,
        value: PrepaidFixedTieredPricingModelResponse,
    ) -> Self {
        self.prepaid_fixed_tiered_pricing_model = Some(value);
        self
    }

    pub fn prepaid_tiered_pricing_model(
        mut self,
        value: PrepaidTieredPricingModelResponse,
    ) -> Self {
        self.prepaid_tiered_pricing_model = Some(value);
        self
    }

    pub fn tiered_percentage_pricing_model(
        mut self,
        value: TieredPercentagePricingModelResponse,
    ) -> Self {
        self.tiered_percentage_pricing_model = Some(value);
        self
    }

    pub fn tiered_pricing_model(mut self, value: TieredPricingModelResponse) -> Self {
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

    pub fn unit_pricing_model(mut self, value: UnitPricingModelResponse) -> Self {
        self.unit_pricing_model = Some(value);
        self
    }

    pub fn volume_percentage_pricing_model(
        mut self,
        value: VolumePercentagePricingModelResponse,
    ) -> Self {
        self.volume_percentage_pricing_model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionPriceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`billing_direction`](SubscriptionPriceResponseBuilder::billing_direction)
    /// - [`charge_on_contract_start`](SubscriptionPriceResponseBuilder::charge_on_contract_start)
    /// - [`currency`](SubscriptionPriceResponseBuilder::currency)
    /// - [`fee_type`](SubscriptionPriceResponseBuilder::fee_type)
    /// - [`r#type`](SubscriptionPriceResponseBuilder::r#type)
    pub fn build(self) -> Result<SubscriptionPriceResponse, BuildError> {
        Ok(SubscriptionPriceResponse {
            billing_direction: self
                .billing_direction
                .ok_or_else(|| BuildError::missing_field("billing_direction"))?,
            billing_frequency: self.billing_frequency,
            billing_interval: self.billing_interval,
            billing_interval_count: self.billing_interval_count,
            billing_interval_unit: self.billing_interval_unit,
            charge_on_contract_start: self
                .charge_on_contract_start
                .ok_or_else(|| BuildError::missing_field("charge_on_contract_start"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
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
