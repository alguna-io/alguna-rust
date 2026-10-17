pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EntitlementInput {
    /// Credit allocation config (required for credit_allocation type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation: Option<EntitlementAllocationInput>,
    /// Entitlement key
    pub entitlement_key: String,
    /// Entitlement mode
    pub mode: EntitlementMode,
    /// Timing config (for grant-based entitlements)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<EntitlementTimingInput>,
    /// Entitlement type
    pub r#type: EntitlementType,
    /// Entitlement value (for boolean/number/string types)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl EntitlementInput {
    pub fn builder() -> EntitlementInputBuilder {
        <EntitlementInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntitlementInputBuilder {
    allocation: Option<EntitlementAllocationInput>,
    entitlement_key: Option<String>,
    mode: Option<EntitlementMode>,
    timing: Option<EntitlementTimingInput>,
    r#type: Option<EntitlementType>,
    value: Option<String>,
}

impl EntitlementInputBuilder {
    pub fn allocation(mut self, value: EntitlementAllocationInput) -> Self {
        self.allocation = Some(value);
        self
    }

    pub fn entitlement_key(mut self, value: impl Into<String>) -> Self {
        self.entitlement_key = Some(value.into());
        self
    }

    pub fn mode(mut self, value: EntitlementMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn timing(mut self, value: EntitlementTimingInput) -> Self {
        self.timing = Some(value);
        self
    }

    pub fn r#type(mut self, value: EntitlementType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EntitlementInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entitlement_key`](EntitlementInputBuilder::entitlement_key)
    /// - [`mode`](EntitlementInputBuilder::mode)
    /// - [`r#type`](EntitlementInputBuilder::r#type)
    pub fn build(self) -> Result<EntitlementInput, BuildError> {
        Ok(EntitlementInput {
            allocation: self.allocation,
            entitlement_key: self
                .entitlement_key
                .ok_or_else(|| BuildError::missing_field("entitlement_key"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            timing: self.timing,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            value: self.value,
        })
    }
}
