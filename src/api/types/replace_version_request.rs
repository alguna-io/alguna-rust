pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReplaceVersionRequest {
    /// Human-readable audit note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated effective date (ISO 8601 date, YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<NaiveDate>,
    /// Full entitlement specification (replaces existing entitlements)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<EntitlementInput>>,
    /// Complete replacement set of items for this draft version.
    pub items: Vec<VersionItemInput>,
}

impl ReplaceVersionRequest {
    pub fn builder() -> ReplaceVersionRequestBuilder {
        <ReplaceVersionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplaceVersionRequestBuilder {
    description: Option<String>,
    effective_at: Option<NaiveDate>,
    entitlements: Option<Vec<EntitlementInput>>,
    items: Option<Vec<VersionItemInput>>,
}

impl ReplaceVersionRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn effective_at(mut self, value: NaiveDate) -> Self {
        self.effective_at = Some(value);
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

    /// Consumes the builder and constructs a [`ReplaceVersionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](ReplaceVersionRequestBuilder::items)
    pub fn build(self) -> Result<ReplaceVersionRequest, BuildError> {
        Ok(ReplaceVersionRequest {
            description: self.description,
            effective_at: self.effective_at,
            entitlements: self.entitlements,
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
