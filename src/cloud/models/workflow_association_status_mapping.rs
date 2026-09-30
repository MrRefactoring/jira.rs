// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowAssociationStatusMapping {
    #[serde(rename = "newStatusId", default, skip_serializing_if = "Option::is_none")]
    pub new_status_id: Option<String>,
    #[serde(rename = "oldStatusId", default, skip_serializing_if = "Option::is_none")]
    pub old_status_id: Option<String>,
}
