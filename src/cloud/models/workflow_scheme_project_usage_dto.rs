// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Projects using the workflow scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowSchemeProjectUsageDTO {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<ProjectUsagePage>,
    /// The workflow scheme ID.
    #[serde(rename = "workflowSchemeId", default, skip_serializing_if = "Option::is_none")]
    pub workflow_scheme_id: Option<String>,
}
