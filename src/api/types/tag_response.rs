pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TagResponse {
    /// Hex color used to render the tag
    pub color: String,
    /// Timestamp when the tag was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Resource types this tag can be attached to
    pub entity_types: Vec<TaggableEntityType>,
    /// Unique identifier for the tag. Pass it to the tag_ids filter on list endpoints
    pub id: String,
    /// Display name of the tag
    pub name: String,
    /// Timestamp when the tag was last updated
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl TagResponse {
    pub fn builder() -> TagResponseBuilder {
        <TagResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TagResponseBuilder {
    color: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    entity_types: Option<Vec<TaggableEntityType>>,
    id: Option<String>,
    name: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl TagResponseBuilder {
    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn entity_types(mut self, value: Vec<TaggableEntityType>) -> Self {
        self.entity_types = Some(value);
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

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TagResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`color`](TagResponseBuilder::color)
    /// - [`created_at`](TagResponseBuilder::created_at)
    /// - [`entity_types`](TagResponseBuilder::entity_types)
    /// - [`id`](TagResponseBuilder::id)
    /// - [`name`](TagResponseBuilder::name)
    /// - [`updated_at`](TagResponseBuilder::updated_at)
    pub fn build(self) -> Result<TagResponse, BuildError> {
        Ok(TagResponse {
            color: self
                .color
                .ok_or_else(|| BuildError::missing_field("color"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            entity_types: self
                .entity_types
                .ok_or_else(|| BuildError::missing_field("entity_types"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
