// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowSchemeRequest {
    #[serde(rename = "defaultWorkflow", default, skip_serializing_if = "Option::is_none")]
    pub default_workflow: Option<bool>,
    #[serde(rename = "issueTypes", default, skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow: Option<String>,
}
