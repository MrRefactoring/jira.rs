// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The payload for creating a workflows. See <https://www.atlassian.com/software/jira/guides/workflows/overview#what-is-a-jira-workflow>
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowCapabilityPayload {
    /// The statuses for the workflow
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<StatusPayload>>,
    #[serde(rename = "workflowScheme", default, skip_serializing_if = "Option::is_none")]
    pub workflow_scheme: Option<WorkflowSchemePayload>,
    /// The transitions for the workflow
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<WorkflowPayload>>,
}
