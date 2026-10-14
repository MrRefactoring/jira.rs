// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Start one agent across multiple issues with a shared optional prompt.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkStartUniformAgentSessionsRequest {
    /// Rovo agent ID.
    #[serde(rename = "agentId")]
    pub agent_id: String,
    /// Caller-supplied UUID.
    #[serde(rename = "clientMutationId")]
    pub client_mutation_id: String,
    /// Between 1 and 50 numeric issue-ID strings. IDs must be unique.
    #[serde(rename = "issueIds")]
    pub issue_ids: Vec<String>,
    /// Optional prompt applied to every issue.
    #[serde(rename = "userPrompt", default, skip_serializing_if = "Option::is_none")]
    pub user_prompt: Option<String>,
}
