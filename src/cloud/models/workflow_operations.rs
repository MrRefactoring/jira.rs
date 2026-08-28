// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Operations allowed on a workflow
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowOperations {
    /// Whether the workflow can be deleted.
    #[serde(rename = "canDelete")]
    pub can_delete: bool,
    /// Whether the workflow can be updated.
    #[serde(rename = "canEdit")]
    pub can_edit: bool,
}
