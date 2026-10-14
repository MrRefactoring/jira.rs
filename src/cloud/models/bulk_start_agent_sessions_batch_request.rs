// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A batch of agent-session invocations.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkStartAgentSessionsBatchRequest {
    /// Caller-supplied UUID
    #[serde(rename = "clientMutationId")]
    pub client_mutation_id: String,
    /// Between 1 and 50 invocations. Request IDs must be unique within the request.
    pub invocations: Vec<BulkAgentSessionInvocation>,
}
