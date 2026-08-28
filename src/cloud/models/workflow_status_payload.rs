// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The statuses to be used in the workflow
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowStatusPayload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<WorkflowStatusLayoutPayload>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pcri: Option<ProjectCreateResourceIdentifier>,
    /// The properties of the workflow status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
