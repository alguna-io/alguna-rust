pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApprovalSubject {
    /// Identifier of the record under approval
    pub id: String,
    /// Kind of record under approval
    pub r#type: String,
}

impl ApprovalSubject {
    pub fn builder() -> ApprovalSubjectBuilder {
        <ApprovalSubjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApprovalSubjectBuilder {
    id: Option<String>,
    r#type: Option<String>,
}

impl ApprovalSubjectBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApprovalSubject`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ApprovalSubjectBuilder::id)
    /// - [`r#type`](ApprovalSubjectBuilder::r#type)
    pub fn build(self) -> Result<ApprovalSubject, BuildError> {
        Ok(ApprovalSubject {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
