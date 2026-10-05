pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PriceCreateRequest {
    /// When the price is billed
    pub billing_direction: BillingDirection,
    /// How often the price is billed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<BillingFrequency>,
    /// Legacy named billing interval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_interval: Option<String>,
    /// Positive count of BillingIntervalUnits per period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_interval_count: Option<i64>,
    /// Unit of the billing cadence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_interval_unit: Option<BillingIntervalUnit>,
    /// Whether to charge on contract start date
    pub charge_on_contract_start: bool,
    /// Discount applied to this price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Discount>,
    /// Display order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i64>,
    /// Expression-based pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_pricing_model: Option<ExpressionPricingModel>,
    /// Whether the price is fixed or metered
    pub fee_type: PricingClass,
    /// Fixed pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_pricing_model: Option<FixedPricingModel>,
    /// Graduated percentage pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduated_percentage_pricing_model: Option<GraduatedPercentagePricingModel>,
    /// Graduated tiered pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduated_tiered_pricing_model: Option<GraduatedTieredPricingModel>,
    /// Matrix pricing details (required when type=matrix)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix_pricing_model: Option<MatrixPricingModel>,
    /// Maximum spend cap
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_spend: Option<SpendingThreshold>,
    /// Associated metric identifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_ids: Option<Vec<String>>,
    /// Minimum spend threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_spend: Option<SpendingThreshold>,
    /// Prepaid fixed tiered pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_fixed_tiered_pricing_model: Option<PrepaidFixedTieredPricingModel>,
    /// Prepaid tiered pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_tiered_pricing_model: Option<PrepaidTieredPricingModel>,
    /// Identifier of the associated product
    pub product_id: String,
    /// Positive count of RecurringInstalmentIntervalUnits per instalment; must divide cleanly into the billing interval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_instalment_interval_count: Option<i64>,
    /// Optional sub-cadence unit splitting the advance charge into instalments within each billing cycle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_instalment_interval_unit: Option<BillingIntervalUnit>,
    /// Tiered percentage pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiered_percentage_pricing_model: Option<TieredPercentagePricingModel>,
    /// Tiered pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiered_pricing_model: Option<TieredPricingModel>,
    /// Trial period in days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    /// The pricing model type
    pub r#type: PricingType,
    /// Unit pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_pricing_model: Option<UnitPricingModel>,
    /// Volume percentage pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_percentage_pricing_model: Option<VolumePercentagePricingModel>,
}

impl PriceCreateRequest {
    pub fn builder() -> PriceCreateRequestBuilder {
        <PriceCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceCreateRequestBuilder {
    billing_direction: Option<BillingDirection>,
    billing_frequency: Option<BillingFrequency>,
    billing_interval: Option<String>,
    billing_interval_count: Option<i64>,
    billing_interval_unit: Option<BillingIntervalUnit>,
    charge_on_contract_start: Option<bool>,
    discount: Option<Discount>,
    display_order: Option<i64>,
    expression_pricing_model: Option<ExpressionPricingModel>,
    fee_type: Option<PricingClass>,
    fixed_pricing_model: Option<FixedPricingModel>,
    graduated_percentage_pricing_model: Option<GraduatedPercentagePricingModel>,
    graduated_tiered_pricing_model: Option<GraduatedTieredPricingModel>,
    matrix_pricing_model: Option<MatrixPricingModel>,
    maximum_spend: Option<SpendingThreshold>,
    metric_ids: Option<Vec<String>>,
    minimum_spend: Option<SpendingThreshold>,
    prepaid_fixed_tiered_pricing_model: Option<PrepaidFixedTieredPricingModel>,
    prepaid_tiered_pricing_model: Option<PrepaidTieredPricingModel>,
    product_id: Option<String>,
    recurring_instalment_interval_count: Option<i64>,
    recurring_instalment_interval_unit: Option<BillingIntervalUnit>,
    tiered_percentage_pricing_model: Option<TieredPercentagePricingModel>,
    tiered_pricing_model: Option<TieredPricingModel>,
    trial_period_days: Option<i64>,
    r#type: Option<PricingType>,
    unit_pricing_model: Option<UnitPricingModel>,
    volume_percentage_pricing_model: Option<VolumePercentagePricingModel>,
}

impl PriceCreateRequestBuilder {
    pub fn billing_direction(mut self, value: BillingDirection) -> Self {
        self.billing_direction = Some(value);
        self
    }

    pub fn billing_frequency(mut self, value: BillingFrequency) -> Self {
        self.billing_frequency = Some(value);
        self
    }

    pub fn billing_interval(mut self, value: impl Into<String>) -> Self {
        self.billing_interval = Some(value.into());
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

    pub fn discount(mut self, value: Discount) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn display_order(mut self, value: i64) -> Self {
        self.display_order = Some(value);
        self
    }

    pub fn expression_pricing_model(mut self, value: ExpressionPricingModel) -> Self {
        self.expression_pricing_model = Some(value);
        self
    }

    pub fn fee_type(mut self, value: PricingClass) -> Self {
        self.fee_type = Some(value);
        self
    }

    pub fn fixed_pricing_model(mut self, value: FixedPricingModel) -> Self {
        self.fixed_pricing_model = Some(value);
        self
    }

    pub fn graduated_percentage_pricing_model(
        mut self,
        value: GraduatedPercentagePricingModel,
    ) -> Self {
        self.graduated_percentage_pricing_model = Some(value);
        self
    }

    pub fn graduated_tiered_pricing_model(mut self, value: GraduatedTieredPricingModel) -> Self {
        self.graduated_tiered_pricing_model = Some(value);
        self
    }

    pub fn matrix_pricing_model(mut self, value: MatrixPricingModel) -> Self {
        self.matrix_pricing_model = Some(value);
        self
    }

    pub fn maximum_spend(mut self, value: SpendingThreshold) -> Self {
        self.maximum_spend = Some(value);
        self
    }

    pub fn metric_ids(mut self, value: Vec<String>) -> Self {
        self.metric_ids = Some(value);
        self
    }

    pub fn minimum_spend(mut self, value: SpendingThreshold) -> Self {
        self.minimum_spend = Some(value);
        self
    }

    pub fn prepaid_fixed_tiered_pricing_model(
        mut self,
        value: PrepaidFixedTieredPricingModel,
    ) -> Self {
        self.prepaid_fixed_tiered_pricing_model = Some(value);
        self
    }

    pub fn prepaid_tiered_pricing_model(mut self, value: PrepaidTieredPricingModel) -> Self {
        self.prepaid_tiered_pricing_model = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
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

    pub fn tiered_percentage_pricing_model(mut self, value: TieredPercentagePricingModel) -> Self {
        self.tiered_percentage_pricing_model = Some(value);
        self
    }

    pub fn tiered_pricing_model(mut self, value: TieredPricingModel) -> Self {
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

    pub fn unit_pricing_model(mut self, value: UnitPricingModel) -> Self {
        self.unit_pricing_model = Some(value);
        self
    }

    pub fn volume_percentage_pricing_model(mut self, value: VolumePercentagePricingModel) -> Self {
        self.volume_percentage_pricing_model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PriceCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`billing_direction`](PriceCreateRequestBuilder::billing_direction)
    /// - [`charge_on_contract_start`](PriceCreateRequestBuilder::charge_on_contract_start)
    /// - [`fee_type`](PriceCreateRequestBuilder::fee_type)
    /// - [`product_id`](PriceCreateRequestBuilder::product_id)
    /// - [`r#type`](PriceCreateRequestBuilder::r#type)
    pub fn build(self) -> Result<PriceCreateRequest, BuildError> {
        Ok(PriceCreateRequest {
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
            product_id: self
                .product_id
                .ok_or_else(|| BuildError::missing_field("product_id"))?,
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
