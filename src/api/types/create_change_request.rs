pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateChangeRequest {
    /// Products or bundles to add
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<ChangeAddItem>>,
    /// Human-readable audit note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Create as draft (true) or auto-publish (false, default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    /// When the change takes effect. One of: 'immediate' (now), 'next_term_renewal' (contract end), 'next_billing_period' (start of the next invoicing cycle), an ISO 8601 date (YYYY-MM-DD), or an RFC3339 timestamp (e.g. 2027-01-01T14:30:00Z).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective: Option<String>,
    /// Swap to a new plan (provides base item set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// Products or bundles to remove
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<ChangeRemoveItem>>,
    /// Version to compute deltas against (default: current active)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version_id: Option<String>,
    /// Products or bundles to modify
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Vec<ChangeUpdateItem>>,
}

impl CreateChangeRequest {
    pub fn builder() -> CreateChangeRequestBuilder {
        <CreateChangeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateChangeRequestBuilder {
    add: Option<Vec<ChangeAddItem>>,
    description: Option<String>,
    draft: Option<bool>,
    effective: Option<String>,
    plan_id: Option<String>,
    remove: Option<Vec<ChangeRemoveItem>>,
    source_version_id: Option<String>,
    update: Option<Vec<ChangeUpdateItem>>,
}

impl CreateChangeRequestBuilder {
    pub fn add(mut self, value: Vec<ChangeAddItem>) -> Self {
        self.add = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn draft(mut self, value: bool) -> Self {
        self.draft = Some(value);
        self
    }

    pub fn effective(mut self, value: impl Into<String>) -> Self {
        self.effective = Some(value.into());
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn remove(mut self, value: Vec<ChangeRemoveItem>) -> Self {
        self.remove = Some(value);
        self
    }

    pub fn source_version_id(mut self, value: impl Into<String>) -> Self {
        self.source_version_id = Some(value.into());
        self
    }

    pub fn update(mut self, value: Vec<ChangeUpdateItem>) -> Self {
        self.update = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateChangeRequest`].
    pub fn build(self) -> Result<CreateChangeRequest, BuildError> {
        Ok(CreateChangeRequest {
            add: self.add,
            description: self.description,
            draft: self.draft,
            effective: self.effective,
            plan_id: self.plan_id,
            remove: self.remove,
            source_version_id: self.source_version_id,
            update: self.update,
        })
    }
}
