pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChangePreviewResponse {
    /// Summary of changes that would be applied
    pub changes_applied: ChangesAppliedResponse,
    /// When the change would take effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<DateTime<FixedOffset>>,
    /// Version the delta was computed against
    pub source_version_id: String,
}

impl ChangePreviewResponse {
    pub fn builder() -> ChangePreviewResponseBuilder {
        <ChangePreviewResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChangePreviewResponseBuilder {
    changes_applied: Option<ChangesAppliedResponse>,
    effective_at: Option<DateTime<FixedOffset>>,
    source_version_id: Option<String>,
}

impl ChangePreviewResponseBuilder {
    pub fn changes_applied(mut self, value: ChangesAppliedResponse) -> Self {
        self.changes_applied = Some(value);
        self
    }

    pub fn effective_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.effective_at = Some(value);
        self
    }

    pub fn source_version_id(mut self, value: impl Into<String>) -> Self {
        self.source_version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChangePreviewResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`changes_applied`](ChangePreviewResponseBuilder::changes_applied)
    /// - [`source_version_id`](ChangePreviewResponseBuilder::source_version_id)
    pub fn build(self) -> Result<ChangePreviewResponse, BuildError> {
        Ok(ChangePreviewResponse {
            changes_applied: self
                .changes_applied
                .ok_or_else(|| BuildError::missing_field("changes_applied"))?,
            effective_at: self.effective_at,
            source_version_id: self
                .source_version_id
                .ok_or_else(|| BuildError::missing_field("source_version_id"))?,
        })
    }
}
