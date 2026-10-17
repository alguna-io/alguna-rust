pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateVersionRequest {
    /// Human-readable audit note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Create as draft (true) or auto-publish (false, default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    /// When the version takes effect (ISO 8601 date, YYYY-MM-DD). Defaults to today.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<String>,
    /// Full entitlement specification for this version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<EntitlementInput>>,
    /// Complete set of items for this version. Every item must have a full price definition.
    pub items: Vec<VersionItemInput>,
}

impl CreateVersionRequest {
    pub fn builder() -> CreateVersionRequestBuilder {
        <CreateVersionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateVersionRequestBuilder {
    description: Option<String>,
    draft: Option<bool>,
    effective_at: Option<String>,
    entitlements: Option<Vec<EntitlementInput>>,
    items: Option<Vec<VersionItemInput>>,
}

impl CreateVersionRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn draft(mut self, value: bool) -> Self {
        self.draft = Some(value);
        self
    }

    pub fn effective_at(mut self, value: impl Into<String>) -> Self {
        self.effective_at = Some(value.into());
        self
    }

    pub fn entitlements(mut self, value: Vec<EntitlementInput>) -> Self {
        self.entitlements = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<VersionItemInput>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateVersionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](CreateVersionRequestBuilder::items)
    pub fn build(self) -> Result<CreateVersionRequest, BuildError> {
        Ok(CreateVersionRequest {
            description: self.description,
            draft: self.draft,
            effective_at: self.effective_at,
            entitlements: self.entitlements,
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
