pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EntitlementResponse {
    /// Credit allocation config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation: Option<EntitlementAllocationResponse>,
    /// Entitlement key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Entitlement mode
    pub mode: EntitlementResponseMode,
    /// Owning subscription ID; null for default-profile entitlements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    /// Timing config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<EntitlementTimingResponse>,
    /// Entitlement type
    pub r#type: EntitlementResponseType,
    /// Entitlement value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Owning subscription version ID; null for default-profile entitlements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl EntitlementResponse {
    pub fn builder() -> EntitlementResponseBuilder {
        <EntitlementResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntitlementResponseBuilder {
    allocation: Option<EntitlementAllocationResponse>,
    key: Option<String>,
    mode: Option<EntitlementResponseMode>,
    subscription_id: Option<String>,
    timing: Option<EntitlementTimingResponse>,
    r#type: Option<EntitlementResponseType>,
    value: Option<String>,
    version_id: Option<String>,
}

impl EntitlementResponseBuilder {
    pub fn allocation(mut self, value: EntitlementAllocationResponse) -> Self {
        self.allocation = Some(value);
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn mode(mut self, value: EntitlementResponseMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    pub fn timing(mut self, value: EntitlementTimingResponse) -> Self {
        self.timing = Some(value);
        self
    }

    pub fn r#type(mut self, value: EntitlementResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EntitlementResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mode`](EntitlementResponseBuilder::mode)
    /// - [`r#type`](EntitlementResponseBuilder::r#type)
    pub fn build(self) -> Result<EntitlementResponse, BuildError> {
        Ok(EntitlementResponse {
            allocation: self.allocation,
            key: self.key,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            subscription_id: self.subscription_id,
            timing: self.timing,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            value: self.value,
            version_id: self.version_id,
        })
    }
}
