pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChangeResponse {
    /// Summary of changes applied
    pub changes_applied: ChangesAppliedResponse,
    /// When the change takes effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<DateTime<FixedOffset>>,
    /// When this version ends, if superseded by a later version. Callers should verify this matches expectations when inserting between existing versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<FixedOffset>>,
    /// Whether a new version was created (false = draft replaced)
    pub is_new_version: bool,
    /// Version the delta was computed against
    pub source_version_id: String,
    /// Created or replaced version ID
    pub version_id: String,
    /// Version status
    pub version_status: SubscriptionPhaseStatus,
}

impl ChangeResponse {
    pub fn builder() -> ChangeResponseBuilder {
        <ChangeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChangeResponseBuilder {
    changes_applied: Option<ChangesAppliedResponse>,
    effective_at: Option<DateTime<FixedOffset>>,
    end_date: Option<DateTime<FixedOffset>>,
    is_new_version: Option<bool>,
    source_version_id: Option<String>,
    version_id: Option<String>,
    version_status: Option<SubscriptionPhaseStatus>,
}

impl ChangeResponseBuilder {
    pub fn changes_applied(mut self, value: ChangesAppliedResponse) -> Self {
        self.changes_applied = Some(value);
        self
    }

    pub fn effective_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.effective_at = Some(value);
        self
    }

    pub fn end_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn is_new_version(mut self, value: bool) -> Self {
        self.is_new_version = Some(value);
        self
    }

    pub fn source_version_id(mut self, value: impl Into<String>) -> Self {
        self.source_version_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn version_status(mut self, value: SubscriptionPhaseStatus) -> Self {
        self.version_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChangeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`changes_applied`](ChangeResponseBuilder::changes_applied)
    /// - [`is_new_version`](ChangeResponseBuilder::is_new_version)
    /// - [`source_version_id`](ChangeResponseBuilder::source_version_id)
    /// - [`version_id`](ChangeResponseBuilder::version_id)
    /// - [`version_status`](ChangeResponseBuilder::version_status)
    pub fn build(self) -> Result<ChangeResponse, BuildError> {
        Ok(ChangeResponse {
            changes_applied: self
                .changes_applied
                .ok_or_else(|| BuildError::missing_field("changes_applied"))?,
            effective_at: self.effective_at,
            end_date: self.end_date,
            is_new_version: self
                .is_new_version
                .ok_or_else(|| BuildError::missing_field("is_new_version"))?,
            source_version_id: self
                .source_version_id
                .ok_or_else(|| BuildError::missing_field("source_version_id"))?,
            version_id: self
                .version_id
                .ok_or_else(|| BuildError::missing_field("version_id"))?,
            version_status: self
                .version_status
                .ok_or_else(|| BuildError::missing_field("version_status"))?,
        })
    }
}
