pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChangesAppliedResponse {
    /// Products or bundles added
    pub added: Vec<ChangeItemRef>,
    /// Products or bundles removed
    pub removed: Vec<ChangeItemRef>,
    /// Products or bundles updated
    pub updated: Vec<ChangeItemRef>,
}

impl ChangesAppliedResponse {
    pub fn builder() -> ChangesAppliedResponseBuilder {
        <ChangesAppliedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChangesAppliedResponseBuilder {
    added: Option<Vec<ChangeItemRef>>,
    removed: Option<Vec<ChangeItemRef>>,
    updated: Option<Vec<ChangeItemRef>>,
}

impl ChangesAppliedResponseBuilder {
    pub fn added(mut self, value: Vec<ChangeItemRef>) -> Self {
        self.added = Some(value);
        self
    }

    pub fn removed(mut self, value: Vec<ChangeItemRef>) -> Self {
        self.removed = Some(value);
        self
    }

    pub fn updated(mut self, value: Vec<ChangeItemRef>) -> Self {
        self.updated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChangesAppliedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`added`](ChangesAppliedResponseBuilder::added)
    /// - [`removed`](ChangesAppliedResponseBuilder::removed)
    /// - [`updated`](ChangesAppliedResponseBuilder::updated)
    pub fn build(self) -> Result<ChangesAppliedResponse, BuildError> {
        Ok(ChangesAppliedResponse {
            added: self
                .added
                .ok_or_else(|| BuildError::missing_field("added"))?,
            removed: self
                .removed
                .ok_or_else(|| BuildError::missing_field("removed"))?,
            updated: self
                .updated
                .ok_or_else(|| BuildError::missing_field("updated"))?,
        })
    }
}
