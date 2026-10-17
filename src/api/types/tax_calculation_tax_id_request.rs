pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxCalculationTaxIdRequest {
    /// Tax id type, e.g. eu_vat, gb_vat, us_ein
    pub r#type: String,
    /// Tax id value
    pub value: String,
}

impl TaxCalculationTaxIdRequest {
    pub fn builder() -> TaxCalculationTaxIdRequestBuilder {
        <TaxCalculationTaxIdRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxCalculationTaxIdRequestBuilder {
    r#type: Option<String>,
    value: Option<String>,
}

impl TaxCalculationTaxIdRequestBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaxCalculationTaxIdRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TaxCalculationTaxIdRequestBuilder::r#type)
    /// - [`value`](TaxCalculationTaxIdRequestBuilder::value)
    pub fn build(self) -> Result<TaxCalculationTaxIdRequest, BuildError> {
        Ok(TaxCalculationTaxIdRequest {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
