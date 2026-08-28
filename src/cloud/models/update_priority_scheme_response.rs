// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of the updated priority scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UpdatePrioritySchemeResponse {
    #[serde(rename = "priorityScheme", default, skip_serializing_if = "Option::is_none")]
    pub priority_scheme: Option<PrioritySchemeWithPaginatedPrioritiesAndProjects>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskProgressJsonNode>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}
