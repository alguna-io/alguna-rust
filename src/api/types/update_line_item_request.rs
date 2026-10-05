pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateLineItemRequest {
    /// Description of the line item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateLineItemRequest {
    pub fn builder() -> UpdateLineItemRequestBuilder {
        <UpdateLineItemRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateLineItemRequestBuilder {
    description: Option<String>,
}

impl UpdateLineItemRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateLineItemRequest`].
    pub fn build(self) -> Result<UpdateLineItemRequest, BuildError> {
        Ok(UpdateLineItemRequest {
            description: self.description,
        })
    }
}
