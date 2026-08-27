// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Properties that identify a workflow.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowId {
    /// **Deprecated:** Whether the workflow is in the draft state. The 'draft' parameter will be removed from this API on [November 2, 2026](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-3147).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    /// The name of the workflow.
    pub name: String,
}
