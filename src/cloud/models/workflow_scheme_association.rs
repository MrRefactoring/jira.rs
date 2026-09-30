// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The explicit association between issue types and a workflow in a workflow scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowSchemeAssociation {
    /// The issue types assigned to the workflow.
    #[serde(rename = "issueTypeIds")]
    pub issue_type_ids: Vec<String>,
    /// The ID of the workflow.
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
}
