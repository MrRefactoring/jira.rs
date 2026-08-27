// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A single entry in the WorkflowHistoryPage.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowHistoryItemDTO {
    /// Whether the version is an intermediate workflow state, sometimes created during workflow updates.
    #[serde(rename = "isIntermediate", default, skip_serializing_if = "Option::is_none")]
    pub is_intermediate: Option<bool>,
    #[serde(rename = "workflowId", default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    #[serde(rename = "workflowVersion", default, skip_serializing_if = "Option::is_none")]
    pub workflow_version: Option<i64>,
    /// The timestamp when this workflow version was created.
    #[serde(rename = "writtenAt", default, skip_serializing_if = "Option::is_none")]
    pub written_at: Option<String>,
}
