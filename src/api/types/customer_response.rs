pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CustomerResponse {
    /// Alternative identifiers for the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// Customer billing address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<AddressResponse>,
    /// Customer contacts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<ContactResponse>>,
    /// Timestamp when the customer was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// ISO 4217 currency code
    pub currency: String,
    /// Unique identifier for the customer
    pub id: String,
    /// Customer name
    pub name: String,
    /// ISO 3166-1 alpha-3 country code used for tax determination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_country: Option<String>,
    /// Customer tax identification number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// Tax rate applied to the customer, expressed as a decimal fraction (0.20 = 20%)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate: Option<String>,
    /// Reason determining how tax is applied to the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reason: Option<TaxabilityReason>,
    /// Customer tax status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_status: Option<String>,
    /// Type of tax applied to the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<TaxType>,
    /// Timestamp when the customer was last updated
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl CustomerResponse {
    pub fn builder() -> CustomerResponseBuilder {
        <CustomerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomerResponseBuilder {
    aliases: Option<Vec<String>>,
    billing_address: Option<AddressResponse>,
    contacts: Option<Vec<ContactResponse>>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    id: Option<String>,
    name: Option<String>,
    tax_country: Option<String>,
    tax_id: Option<String>,
    tax_rate: Option<String>,
    tax_reason: Option<TaxabilityReason>,
    tax_status: Option<String>,
    tax_type: Option<TaxType>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl CustomerResponseBuilder {
    pub fn aliases(mut self, value: Vec<String>) -> Self {
        self.aliases = Some(value);
        self
    }

    pub fn billing_address(mut self, value: AddressResponse) -> Self {
        self.billing_address = Some(value);
        self
    }

    pub fn contacts(mut self, value: Vec<ContactResponse>) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tax_country(mut self, value: impl Into<String>) -> Self {
        self.tax_country = Some(value.into());
        self
    }

    pub fn tax_id(mut self, value: impl Into<String>) -> Self {
        self.tax_id = Some(value.into());
        self
    }

    pub fn tax_rate(mut self, value: impl Into<String>) -> Self {
        self.tax_rate = Some(value.into());
        self
    }

    pub fn tax_reason(mut self, value: TaxabilityReason) -> Self {
        self.tax_reason = Some(value);
        self
    }

    pub fn tax_status(mut self, value: impl Into<String>) -> Self {
        self.tax_status = Some(value.into());
        self
    }

    pub fn tax_type(mut self, value: TaxType) -> Self {
        self.tax_type = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](CustomerResponseBuilder::created_at)
    /// - [`currency`](CustomerResponseBuilder::currency)
    /// - [`id`](CustomerResponseBuilder::id)
    /// - [`name`](CustomerResponseBuilder::name)
    /// - [`updated_at`](CustomerResponseBuilder::updated_at)
    pub fn build(self) -> Result<CustomerResponse, BuildError> {
        Ok(CustomerResponse {
            aliases: self.aliases,
            billing_address: self.billing_address,
            contacts: self.contacts,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            tax_country: self.tax_country,
            tax_id: self.tax_id,
            tax_rate: self.tax_rate,
            tax_reason: self.tax_reason,
            tax_status: self.tax_status,
            tax_type: self.tax_type,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
