pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApprovalFlowPayload {
    /// When the flow reached its terminal state, RFC 3339
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<FixedOffset>>,
    /// When the flow started, RFC 3339
    #[serde(rename = "createdAt")]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Unique identifier for the approval flow
    pub id: String,
    /// Status of the operation the flow gates
    #[serde(rename = "outcomeStatus")]
    pub outcome_status: ApprovalFlowPayloadOutcomeStatus,
    /// Quote the flow ran against. Present only when the flow belongs to a quote.
    #[serde(rename = "quoteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    /// Why the flow was canceled. Null unless status is canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Approval scenario the flow was evaluated for. Empty for flows created before scenarios existed.
    pub scenario: String,
    /// Terminal status the flow reached
    pub status: ApprovalFlowPayloadStatus,
    /// Record under approval
    pub subject: ApprovalSubject,
    /// Subscription the flow ran against
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
    /// Subscription version the flow ran against
    #[serde(rename = "subscriptionVersionId")]
    pub subscription_version_id: String,
    /// When the flow was last updated, RFC 3339
    #[serde(rename = "updatedAt")]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl ApprovalFlowPayload {
    pub fn builder() -> ApprovalFlowPayloadBuilder {
        <ApprovalFlowPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApprovalFlowPayloadBuilder {
    completed_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    outcome_status: Option<ApprovalFlowPayloadOutcomeStatus>,
    quote_id: Option<String>,
    reason: Option<String>,
    scenario: Option<String>,
    status: Option<ApprovalFlowPayloadStatus>,
    subject: Option<ApprovalSubject>,
    subscription_id: Option<String>,
    subscription_version_id: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ApprovalFlowPayloadBuilder {
    pub fn completed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn outcome_status(mut self, value: ApprovalFlowPayloadOutcomeStatus) -> Self {
        self.outcome_status = Some(value);
        self
    }

    pub fn quote_id(mut self, value: impl Into<String>) -> Self {
        self.quote_id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn scenario(mut self, value: impl Into<String>) -> Self {
        self.scenario = Some(value.into());
        self
    }

    pub fn status(mut self, value: ApprovalFlowPayloadStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subject(mut self, value: ApprovalSubject) -> Self {
        self.subject = Some(value);
        self
    }

    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    pub fn subscription_version_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_version_id = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApprovalFlowPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ApprovalFlowPayloadBuilder::created_at)
    /// - [`id`](ApprovalFlowPayloadBuilder::id)
    /// - [`outcome_status`](ApprovalFlowPayloadBuilder::outcome_status)
    /// - [`scenario`](ApprovalFlowPayloadBuilder::scenario)
    /// - [`status`](ApprovalFlowPayloadBuilder::status)
    /// - [`subject`](ApprovalFlowPayloadBuilder::subject)
    /// - [`subscription_id`](ApprovalFlowPayloadBuilder::subscription_id)
    /// - [`subscription_version_id`](ApprovalFlowPayloadBuilder::subscription_version_id)
    /// - [`updated_at`](ApprovalFlowPayloadBuilder::updated_at)
    pub fn build(self) -> Result<ApprovalFlowPayload, BuildError> {
        Ok(ApprovalFlowPayload {
            completed_at: self.completed_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            outcome_status: self
                .outcome_status
                .ok_or_else(|| BuildError::missing_field("outcome_status"))?,
            quote_id: self.quote_id,
            reason: self.reason,
            scenario: self
                .scenario
                .ok_or_else(|| BuildError::missing_field("scenario"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            subject: self
                .subject
                .ok_or_else(|| BuildError::missing_field("subject"))?,
            subscription_id: self
                .subscription_id
                .ok_or_else(|| BuildError::missing_field("subscription_id"))?,
            subscription_version_id: self
                .subscription_version_id
                .ok_or_else(|| BuildError::missing_field("subscription_version_id"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
