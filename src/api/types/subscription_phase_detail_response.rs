pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscriptionPhaseDetailResponse {
    /// When the version was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Change description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Version end date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<FixedOffset>>,
    /// Version entitlements
    pub entitlements: Vec<EntitlementResponse>,
    /// Version ID
    pub id: String,
    /// Version items
    pub items: Vec<SubscriptionPhaseItemResponse>,
    /// Plan ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// Version start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<FixedOffset>>,
    /// Version status
    pub status: SubscriptionPhaseDetailResponseStatus,
    /// Subscription ID
    pub subscription_id: String,
    /// When the version was last updated
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl SubscriptionPhaseDetailResponse {
    pub fn builder() -> SubscriptionPhaseDetailResponseBuilder {
        <SubscriptionPhaseDetailResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionPhaseDetailResponseBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    end_date: Option<DateTime<FixedOffset>>,
    entitlements: Option<Vec<EntitlementResponse>>,
    id: Option<String>,
    items: Option<Vec<SubscriptionPhaseItemResponse>>,
    plan_id: Option<String>,
    start_date: Option<DateTime<FixedOffset>>,
    status: Option<SubscriptionPhaseDetailResponseStatus>,
    subscription_id: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl SubscriptionPhaseDetailResponseBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn entitlements(mut self, value: Vec<EntitlementResponse>) -> Self {
        self.entitlements = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<SubscriptionPhaseItemResponse>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn start_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn status(mut self, value: SubscriptionPhaseDetailResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionPhaseDetailResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](SubscriptionPhaseDetailResponseBuilder::created_at)
    /// - [`entitlements`](SubscriptionPhaseDetailResponseBuilder::entitlements)
    /// - [`id`](SubscriptionPhaseDetailResponseBuilder::id)
    /// - [`items`](SubscriptionPhaseDetailResponseBuilder::items)
    /// - [`status`](SubscriptionPhaseDetailResponseBuilder::status)
    /// - [`subscription_id`](SubscriptionPhaseDetailResponseBuilder::subscription_id)
    /// - [`updated_at`](SubscriptionPhaseDetailResponseBuilder::updated_at)
    pub fn build(self) -> Result<SubscriptionPhaseDetailResponse, BuildError> {
        Ok(SubscriptionPhaseDetailResponse {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self.description,
            end_date: self.end_date,
            entitlements: self
                .entitlements
                .ok_or_else(|| BuildError::missing_field("entitlements"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
            plan_id: self.plan_id,
            start_date: self.start_date,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            subscription_id: self
                .subscription_id
                .ok_or_else(|| BuildError::missing_field("subscription_id"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
