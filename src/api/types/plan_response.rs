pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlanResponse {
    /// Timestamp when the plan was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The currency in which the plan is billed
    pub currency: String,
    /// A brief description of the plan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Unique identifier for the plan
    pub id: String,
    /// The name of the plan
    pub name: String,
    /// Timestamp when the plan was last updated
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl PlanResponse {
    pub fn builder() -> PlanResponseBuilder {
        <PlanResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlanResponseBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    description: Option<String>,
    id: Option<String>,
    name: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl PlanResponseBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlanResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](PlanResponseBuilder::created_at)
    /// - [`currency`](PlanResponseBuilder::currency)
    /// - [`id`](PlanResponseBuilder::id)
    /// - [`name`](PlanResponseBuilder::name)
    /// - [`updated_at`](PlanResponseBuilder::updated_at)
    pub fn build(self) -> Result<PlanResponse, BuildError> {
        Ok(PlanResponse {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            description: self.description,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
