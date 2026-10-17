pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePlanInvoiceSettingsInput {
    /// Whether to auto-issue invoices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_issue_invoices: Option<bool>,
    /// Whether to auto-pay invoices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay_invoices: Option<bool>,
    /// Footer text applied to invoices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_footer_text: Option<String>,
    /// Template applied to invoice memos
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_memo_template: Option<String>,
    /// Whether to send an activation email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_activation_email: Option<bool>,
    /// Whether to email invoices to the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_invoices_to_customer: Option<bool>,
    /// Whether to email receipts to the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_receipts_to_customer: Option<bool>,
}

impl CreatePlanInvoiceSettingsInput {
    pub fn builder() -> CreatePlanInvoiceSettingsInputBuilder {
        <CreatePlanInvoiceSettingsInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePlanInvoiceSettingsInputBuilder {
    auto_issue_invoices: Option<bool>,
    auto_pay_invoices: Option<bool>,
    invoice_footer_text: Option<String>,
    invoice_memo_template: Option<String>,
    send_activation_email: Option<bool>,
    send_invoices_to_customer: Option<bool>,
    send_receipts_to_customer: Option<bool>,
}

impl CreatePlanInvoiceSettingsInputBuilder {
    pub fn auto_issue_invoices(mut self, value: bool) -> Self {
        self.auto_issue_invoices = Some(value);
        self
    }

    pub fn auto_pay_invoices(mut self, value: bool) -> Self {
        self.auto_pay_invoices = Some(value);
        self
    }

    pub fn invoice_footer_text(mut self, value: impl Into<String>) -> Self {
        self.invoice_footer_text = Some(value.into());
        self
    }

    pub fn invoice_memo_template(mut self, value: impl Into<String>) -> Self {
        self.invoice_memo_template = Some(value.into());
        self
    }

    pub fn send_activation_email(mut self, value: bool) -> Self {
        self.send_activation_email = Some(value);
        self
    }

    pub fn send_invoices_to_customer(mut self, value: bool) -> Self {
        self.send_invoices_to_customer = Some(value);
        self
    }

    pub fn send_receipts_to_customer(mut self, value: bool) -> Self {
        self.send_receipts_to_customer = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePlanInvoiceSettingsInput`].
    pub fn build(self) -> Result<CreatePlanInvoiceSettingsInput, BuildError> {
        Ok(CreatePlanInvoiceSettingsInput {
            auto_issue_invoices: self.auto_issue_invoices,
            auto_pay_invoices: self.auto_pay_invoices,
            invoice_footer_text: self.invoice_footer_text,
            invoice_memo_template: self.invoice_memo_template,
            send_activation_email: self.send_activation_email,
            send_invoices_to_customer: self.send_invoices_to_customer,
            send_receipts_to_customer: self.send_receipts_to_customer,
        })
    }
}
