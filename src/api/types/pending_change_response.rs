pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PendingChangeResponse {
    /// Change description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When the change takes effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<DateTime<FixedOffset>>,
    /// Version status
    pub status: PendingChangeResponseStatus,
    /// Version ID
    pub version_id: String,
}

impl PendingChangeResponse {
    pub fn builder() -> PendingChangeResponseBuilder {
        <PendingChangeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PendingChangeResponseBuilder {
    description: Option<String>,
    effective_at: Option<DateTime<FixedOffset>>,
    status: Option<PendingChangeResponseStatus>,
    version_id: Option<String>,
}

impl PendingChangeResponseBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn effective_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.effective_at = Some(value);
        self
    }

    pub fn status(mut self, value: PendingChangeResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PendingChangeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](PendingChangeResponseBuilder::status)
    /// - [`version_id`](PendingChangeResponseBuilder::version_id)
    pub fn build(self) -> Result<PendingChangeResponse, BuildError> {
        Ok(PendingChangeResponse {
            description: self.description,
            effective_at: self.effective_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            version_id: self
                .version_id
                .ok_or_else(|| BuildError::missing_field("version_id"))?,
        })
    }
}
