// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SharePermissionInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groupname: Option<String>,
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "projectRoleId", default, skip_serializing_if = "Option::is_none")]
    pub project_role_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "userKey", default, skip_serializing_if = "Option::is_none")]
    pub user_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<bool>,
}
