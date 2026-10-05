pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCustomerPortalSessionRequest {
    /// Settings for the billing details section of the customer portal. When omitted, the organization's defaults apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details_settings: Option<BillingDetailsSettingsRequest>,
    /// Alternative identifier (external ID) for the customer. Used when customer_id is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_alias: Option<String>,
    /// Customer ID the portal session is for. Either customer_id or customer_alias must be provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Settings for the invoice list section of the customer portal. When omitted, the organization's defaults apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_list_settings: Option<InvoiceListSettingsRequest>,
    /// Settings for the plan overview section of the customer portal. When omitted, the organization's defaults apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_overview_settings: Option<PlanOverviewSettingsRequest>,
}

impl CreateCustomerPortalSessionRequest {
    pub fn builder() -> CreateCustomerPortalSessionRequestBuilder {
        <CreateCustomerPortalSessionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCustomerPortalSessionRequestBuilder {
    billing_details_settings: Option<BillingDetailsSettingsRequest>,
    customer_alias: Option<String>,
    customer_id: Option<String>,
    invoice_list_settings: Option<InvoiceListSettingsRequest>,
    plan_overview_settings: Option<PlanOverviewSettingsRequest>,
}

impl CreateCustomerPortalSessionRequestBuilder {
    pub fn billing_details_settings(mut self, value: BillingDetailsSettingsRequest) -> Self {
        self.billing_details_settings = Some(value);
        self
    }

    pub fn customer_alias(mut self, value: impl Into<String>) -> Self {
        self.customer_alias = Some(value.into());
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn invoice_list_settings(mut self, value: InvoiceListSettingsRequest) -> Self {
        self.invoice_list_settings = Some(value);
        self
    }

    pub fn plan_overview_settings(mut self, value: PlanOverviewSettingsRequest) -> Self {
        self.plan_overview_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCustomerPortalSessionRequest`].
    pub fn build(self) -> Result<CreateCustomerPortalSessionRequest, BuildError> {
        Ok(CreateCustomerPortalSessionRequest {
            billing_details_settings: self.billing_details_settings,
            customer_alias: self.customer_alias,
            customer_id: self.customer_id,
            invoice_list_settings: self.invoice_list_settings,
            plan_overview_settings: self.plan_overview_settings,
        })
    }
}
