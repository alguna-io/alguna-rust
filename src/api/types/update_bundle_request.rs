pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateBundleRequest {
    /// A brief description of the bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Prices to include in this bundle (full replacement)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<PriceCreateRequest>>,
}

impl UpdateBundleRequest {
    pub fn builder() -> UpdateBundleRequestBuilder {
        <UpdateBundleRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBundleRequestBuilder {
    description: Option<String>,
    name: Option<String>,
    prices: Option<Vec<PriceCreateRequest>>,
}

impl UpdateBundleRequestBuilder {
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

    /// Consumes the builder and constructs a [`UpdateBundleRequest`].
    pub fn build(self) -> Result<UpdateBundleRequest, BuildError> {
        Ok(UpdateBundleRequest {
            description: self.description,
            name: self.name,
            prices: self.prices,
        })
    }
}
