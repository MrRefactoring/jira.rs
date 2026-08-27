// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The preview workflow response containing workflows and statuses.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowPreviewResponse {
    /// The list of statuses referenced by the workflows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<JiraWorkflowPreviewStatus>>,
    /// The list of workflows. The workflows are returned in the same order as specified in the request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<WorkflowPreview>>,
}
