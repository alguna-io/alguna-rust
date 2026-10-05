pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateBundleRequest {
    /// A brief description of the bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the bundle
    pub name: String,
    /// Prices to include in this bundle
    pub prices: Vec<PriceCreateRequest>,
}

impl CreateBundleRequest {
    pub fn builder() -> CreateBundleRequestBuilder {
        <CreateBundleRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBundleRequestBuilder {
    description: Option<String>,
    name: Option<String>,
    prices: Option<Vec<PriceCreateRequest>>,
}

impl CreateBundleRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn prices(mut self, value: Vec<PriceCreateRequest>) -> Self {
        self.prices = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBundleRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateBundleRequestBuilder::name)
    /// - [`prices`](CreateBundleRequestBuilder::prices)
    pub fn build(self) -> Result<CreateBundleRequest, BuildError> {
        Ok(CreateBundleRequest {
            description: self.description,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            prices: self
                .prices
                .ok_or_else(|| BuildError::missing_field("prices"))?,
        })
    }
}
