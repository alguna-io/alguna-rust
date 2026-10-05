pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlanOverviewSettingsRequest {
    /// Whether the customer can change their plan self-serve. Falls back to the organization's default when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_self_serve_change: Option<bool>,
    /// Whether the customer can cancel their subscription. Falls back to the organization's default when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_subscription_cancel: Option<bool>,
    /// Whether the plan overview section is shown. Falls back to the organization's default when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_section: Option<bool>,
}

impl PlanOverviewSettingsRequest {
    pub fn builder() -> PlanOverviewSettingsRequestBuilder {
        <PlanOverviewSettingsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlanOverviewSettingsRequestBuilder {
    allow_self_serve_change: Option<bool>,
    allow_subscription_cancel: Option<bool>,
    show_section: Option<bool>,
}

impl PlanOverviewSettingsRequestBuilder {
    pub fn allow_self_serve_change(mut self, value: bool) -> Self {
        self.allow_self_serve_change = Some(value);
        self
    }

    pub fn allow_subscription_cancel(mut self, value: bool) -> Self {
        self.allow_subscription_cancel = Some(value);
        self
    }

    pub fn show_section(mut self, value: bool) -> Self {
        self.show_section = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlanOverviewSettingsRequest`].
    pub fn build(self) -> Result<PlanOverviewSettingsRequest, BuildError> {
        Ok(PlanOverviewSettingsRequest {
            allow_self_serve_change: self.allow_self_serve_change,
            allow_subscription_cancel: self.allow_subscription_cancel,
            show_section: self.show_section,
        })
    }
}
