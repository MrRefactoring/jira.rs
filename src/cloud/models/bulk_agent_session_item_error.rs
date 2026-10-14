// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Error type.
    pub enum BulkAgentSessionItemErrorType {
        AgentNotAssignable => "AGENT_NOT_ASSIGNABLE",
        AgentNotFound => "AGENT_NOT_FOUND",
        InvalidRequestId => "INVALID_REQUEST_ID",
        IssueNotFound => "ISSUE_NOT_FOUND",
        IdempotencyConflict => "IDEMPOTENCY_CONFLICT",
        IssueVersionMissing => "ISSUE_VERSION_MISSING",
        IssueAccessDenied => "ISSUE_ACCESS_DENIED",
        RateLimited => "RATE_LIMITED",
        ConcurrencySaturated => "CONCURRENCY_SATURATED",
        AgentTriggerFailed => "AGENT_TRIGGER_FAILED",
        IdempotencyError => "IDEMPOTENCY_ERROR",
        PriorAttemptFailed => "PRIOR_ATTEMPT_FAILED",
        SessionNotStarted => "SESSION_NOT_STARTED",
        InDoubt => "IN_DOUBT",
        UnexpectedError => "UNEXPECTED_ERROR",
    }
}

/// An item-level error inside an HTTP 200 response.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkAgentSessionItemError {
    /// Error message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP status code.
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
    /// Error type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<BulkAgentSessionItemErrorType>,
}
