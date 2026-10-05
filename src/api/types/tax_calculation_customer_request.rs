pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxCalculationCustomerRequest {
    /// Customer address used for tax determination
    pub address: TaxCalculationAddressRequest,
    /// Customer email; the receipt recipient, frozen for issuance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Customer first name; frozen for issuance to name/contact the account
    pub first_name: String,
    /// Customer last name; frozen for issuance to name/contact the account
    pub last_name: String,
    /// Customer reference (e.g. your customer id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Customer tax ids for reverse-charge / exemption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<TaxCalculationTaxIdRequest>>,
}

impl TaxCalculationCustomerRequest {
    pub fn builder() -> TaxCalculationCustomerRequestBuilder {
        <TaxCalculationCustomerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxCalculationCustomerRequestBuilder {
    address: Option<TaxCalculationAddressRequest>,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    reference: Option<String>,
    tax_ids: Option<Vec<TaxCalculationTaxIdRequest>>,
}

impl TaxCalculationCustomerRequestBuilder {
    pub fn address(mut self, value: TaxCalculationAddressRequest) -> Self {
        self.address = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn tax_ids(mut self, value: Vec<TaxCalculationTaxIdRequest>) -> Self {
        self.tax_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaxCalculationCustomerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`address`](TaxCalculationCustomerRequestBuilder::address)
    /// - [`first_name`](TaxCalculationCustomerRequestBuilder::first_name)
    /// - [`last_name`](TaxCalculationCustomerRequestBuilder::last_name)
    pub fn build(self) -> Result<TaxCalculationCustomerRequest, BuildError> {
        Ok(TaxCalculationCustomerRequest {
            address: self
                .address
                .ok_or_else(|| BuildError::missing_field("address"))?,
            email: self.email,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            reference: self.reference,
            tax_ids: self.tax_ids,
        })
    }
}
