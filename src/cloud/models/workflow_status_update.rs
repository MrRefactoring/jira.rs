// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The category of the status.
    pub enum WorkflowStatusUpdateStatusCategory {
        Todo => "TODO",
        InProgress => "IN_PROGRESS",
        Done => "DONE",
    }
}

/// Details of the status being updated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowStatusUpdate {
    /// The description of the status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the status. When reusing an existing status, this field should be provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the status.
    pub name: String,
    /// The category of the status.
    #[serde(rename = "statusCategory")]
    pub status_category: WorkflowStatusUpdateStatusCategory,
    /// The reference of the status. If adding a new status to a team-managed workflow, this must be a UUID (for company-managed a UUID is not needed).
    #[serde(rename = "statusReference")]
    pub status_reference: String,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}
