// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// One agent-session invocation for an issue.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkAgentSessionInvocation {
    /// Rovo agent ID.
    #[serde(rename = "agentId")]
    pub agent_id: String,
    /// Numeric issue ID as a string.
    #[serde(rename = "issueId")]
    pub issue_id: String,
    /// Unique per-item UUID.
    #[serde(rename = "requestId")]
    pub request_id: String,
    /// Optional prompt for this invocation.
    #[serde(rename = "userPrompt", default, skip_serializing_if = "Option::is_none")]
    pub user_prompt: Option<String>,
}
