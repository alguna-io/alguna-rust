pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BundleResponse {
    /// A brief description of the bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Unique identifier for the bundle
    pub id: String,
    /// The name of the bundle
    pub name: String,
    /// Prices included in this bundle
    pub prices: Vec<PriceResponse>,
}

impl BundleResponse {
    pub fn builder() -> BundleResponseBuilder {
        <BundleResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BundleResponseBuilder {
    description: Option<String>,
    id: Option<String>,
    name: Option<String>,
    prices: Option<Vec<PriceResponse>>,
}

impl BundleResponseBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
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

    pub fn prices(mut self, value: Vec<PriceResponse>) -> Self {
        self.prices = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BundleResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BundleResponseBuilder::id)
    /// - [`name`](BundleResponseBuilder::name)
    /// - [`prices`](BundleResponseBuilder::prices)
    pub fn build(self) -> Result<BundleResponse, BuildError> {
        Ok(BundleResponse {
            description: self.description,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            prices: self
                .prices
                .ok_or_else(|| BuildError::missing_field("prices"))?,
        })
    }
}
