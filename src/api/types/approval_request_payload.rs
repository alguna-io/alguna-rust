pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApprovalRequestPayload {
    /// Approval flow the request belongs to
    #[serde(rename = "approvalFlowId")]
    pub approval_flow_id: String,
    /// Email of the approver the request was raised with
    #[serde(rename = "approverEmail")]
    pub approver_email: String,
    /// Name of the approver the request was raised with
    #[serde(rename = "approverName")]
    pub approver_name: String,
    /// When the request was raised, RFC 3339
    #[serde(rename = "createdAt")]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// When the decision was made, RFC 3339. Null while pending.
    #[serde(rename = "decisionMadeAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_made_at: Option<DateTime<FixedOffset>>,
    /// Reason the approver gave for their decision. Null while pending.
    #[serde(rename = "decisionReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_reason: Option<String>,
    /// Unique identifier for the approval request
    pub id: String,
    /// Status of the operation the flow gates
    #[serde(rename = "outcomeStatus")]
    pub outcome_status: ApprovalRequestPayloadOutcomeStatus,
    /// Quote under approval. Present only when the request belongs to a quote.
    #[serde(rename = "quoteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    /// Approval rule that produced this request
    #[serde(rename = "ruleName")]
    pub rule_name: String,
    /// Approval scenario the flow was evaluated for. Empty for flows created before scenarios existed.
    pub scenario: String,
    /// Request status
    pub status: ApprovalStatus,
    /// Record under approval
    pub subject: ApprovalSubject,
    /// Subscription under approval
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
    /// Subscription version under approval
    #[serde(rename = "subscriptionVersionId")]
    pub subscription_version_id: String,
    /// When the request was last updated, RFC 3339
    #[serde(rename = "updatedAt")]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl ApprovalRequestPayload {
    pub fn builder() -> ApprovalRequestPayloadBuilder {
        <ApprovalRequestPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApprovalRequestPayloadBuilder {
    approval_flow_id: Option<String>,
    approver_email: Option<String>,
    approver_name: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    decision_made_at: Option<DateTime<FixedOffset>>,
    decision_reason: Option<String>,
    id: Option<String>,
    outcome_status: Option<ApprovalRequestPayloadOutcomeStatus>,
    quote_id: Option<String>,
    rule_name: Option<String>,
    scenario: Option<String>,
    status: Option<ApprovalStatus>,
    subject: Option<ApprovalSubject>,
    subscription_id: Option<String>,
    subscription_version_id: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ApprovalRequestPayloadBuilder {
    pub fn approval_flow_id(mut self, value: impl Into<String>) -> Self {
        self.approval_flow_id = Some(value.into());
        self
    }

    pub fn approver_email(mut self, value: impl Into<String>) -> Self {
        self.approver_email = Some(value.into());
        self
    }

    pub fn approver_name(mut self, value: impl Into<String>) -> Self {
        self.approver_name = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn decision_made_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.decision_made_at = Some(value);
        self
    }

    pub fn decision_reason(mut self, value: impl Into<String>) -> Self {
        self.decision_reason = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn outcome_status(mut self, value: ApprovalRequestPayloadOutcomeStatus) -> Self {
        self.outcome_status = Some(value);
        self
    }

    pub fn quote_id(mut self, value: impl Into<String>) -> Self {
        self.quote_id = Some(value.into());
        self
    }

    pub fn rule_name(mut self, value: impl Into<String>) -> Self {
        self.rule_name = Some(value.into());
        self
    }

    pub fn scenario(mut self, value: impl Into<String>) -> Self {
        self.scenario = Some(value.into());
        self
    }

    pub fn status(mut self, value: ApprovalStatus) -> Self {
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

    /// Consumes the builder and constructs a [`ApprovalRequestPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`approval_flow_id`](ApprovalRequestPayloadBuilder::approval_flow_id)
    /// - [`approver_email`](ApprovalRequestPayloadBuilder::approver_email)
    /// - [`approver_name`](ApprovalRequestPayloadBuilder::approver_name)
    /// - [`created_at`](ApprovalRequestPayloadBuilder::created_at)
    /// - [`id`](ApprovalRequestPayloadBuilder::id)
    /// - [`outcome_status`](ApprovalRequestPayloadBuilder::outcome_status)
    /// - [`rule_name`](ApprovalRequestPayloadBuilder::rule_name)
    /// - [`scenario`](ApprovalRequestPayloadBuilder::scenario)
    /// - [`status`](ApprovalRequestPayloadBuilder::status)
    /// - [`subject`](ApprovalRequestPayloadBuilder::subject)
    /// - [`subscription_id`](ApprovalRequestPayloadBuilder::subscription_id)
    /// - [`subscription_version_id`](ApprovalRequestPayloadBuilder::subscription_version_id)
    /// - [`updated_at`](ApprovalRequestPayloadBuilder::updated_at)
    pub fn build(self) -> Result<ApprovalRequestPayload, BuildError> {
        Ok(ApprovalRequestPayload {
            approval_flow_id: self
                .approval_flow_id
                .ok_or_else(|| BuildError::missing_field("approval_flow_id"))?,
            approver_email: self
                .approver_email
                .ok_or_else(|| BuildError::missing_field("approver_email"))?,
            approver_name: self
                .approver_name
                .ok_or_else(|| BuildError::missing_field("approver_name"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            decision_made_at: self.decision_made_at,
            decision_reason: self.decision_reason,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            outcome_status: self
                .outcome_status
                .ok_or_else(|| BuildError::missing_field("outcome_status"))?,
            quote_id: self.quote_id,
            rule_name: self
                .rule_name
                .ok_or_else(|| BuildError::missing_field("rule_name"))?,
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
