// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RemoveWorkflowRequest {
    #[serde(rename = "nextDefaultWorkflow", default, skip_serializing_if = "Option::is_none")]
    pub next_default_workflow: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow: Option<String>,
}
