// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The issue type.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowProjectIssueTypeUsage {
    /// The ID of the issue type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
