pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntegrationResponse {
    /// Timestamp when the integration was connected
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Human-readable label distinguishing multiple integrations of the same provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Unique identifier for the integration
    pub id: String,
    /// The integration provider
    pub provider: String,
}

impl IntegrationResponse {
    pub fn builder() -> IntegrationResponseBuilder {
        <IntegrationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntegrationResponseBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    id: Option<String>,
    provider: Option<String>,
}

impl IntegrationResponseBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
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

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IntegrationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](IntegrationResponseBuilder::created_at)
    /// - [`id`](IntegrationResponseBuilder::id)
    /// - [`provider`](IntegrationResponseBuilder::provider)
    pub fn build(self) -> Result<IntegrationResponse, BuildError> {
        Ok(IntegrationResponse {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self.description,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}
