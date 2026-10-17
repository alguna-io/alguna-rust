pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BillingDetailsSettingsRequest {
    /// Whether the customer can edit their account details. Falls back to the organization's default when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_account_edit: Option<bool>,
    /// Whether the customer can edit autopay settings. Falls back to the organization's default when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_autopay_edit: Option<bool>,
    /// Whether the customer can edit their payment method. Falls back to the organization's default when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_payment_method_edit: Option<bool>,
    /// Whether the billing details section is shown. Falls back to the organization's default when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_section: Option<bool>,
}

impl BillingDetailsSettingsRequest {
    pub fn builder() -> BillingDetailsSettingsRequestBuilder {
        <BillingDetailsSettingsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingDetailsSettingsRequestBuilder {
    allow_account_edit: Option<bool>,
    allow_autopay_edit: Option<bool>,
    allow_payment_method_edit: Option<bool>,
    show_section: Option<bool>,
}

impl BillingDetailsSettingsRequestBuilder {
    pub fn allow_account_edit(mut self, value: bool) -> Self {
        self.allow_account_edit = Some(value);
        self
    }

    pub fn allow_autopay_edit(mut self, value: bool) -> Self {
        self.allow_autopay_edit = Some(value);
        self
    }

    pub fn allow_payment_method_edit(mut self, value: bool) -> Self {
        self.allow_payment_method_edit = Some(value);
        self
    }

    pub fn show_section(mut self, value: bool) -> Self {
        self.show_section = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillingDetailsSettingsRequest`].
    pub fn build(self) -> Result<BillingDetailsSettingsRequest, BuildError> {
        Ok(BillingDetailsSettingsRequest {
            allow_account_edit: self.allow_account_edit,
            allow_autopay_edit: self.allow_autopay_edit,
            allow_payment_method_edit: self.allow_payment_method_edit,
            show_section: self.show_section,
        })
    }
}
