// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// STARTED: session started; ALREADY\_STARTED: recorded successful retry; FAILED: invocation failed; RATE\_LIMITED: admission denied; IN\_DOUBT: start could not be confirmed.
    pub enum BulkAgentSessionOutcomeResult {
        Started => "STARTED",
        AlreadyStarted => "ALREADY_STARTED",
        Failed => "FAILED",
        RateLimited => "RATE_LIMITED",
        InDoubt => "IN_DOUBT",
    }
}

/// The start outcome for one invocation.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkAgentSessionOutcome {
    /// The caller's Rovo agent ID.
    #[serde(rename = "agentId", default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Conversation ID for STARTED or ALREADY\_STARTED only.
    #[serde(rename = "conversationId", default, skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<BulkAgentSessionItemError>,
    /// Numeric issue ID.
    #[serde(rename = "issueId", default, skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
    /// The per-item requestId for batch; the shared clientMutationId for uniform.
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// STARTED: session started; ALREADY\_STARTED: recorded successful retry; FAILED: invocation failed; RATE\_LIMITED: admission denied; IN\_DOUBT: start could not be confirmed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<BulkAgentSessionOutcomeResult>,
}
