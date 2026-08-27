// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// List of permission grants
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PermissionGrantDTO {
    #[serde(rename = "applicationAccess", default, skip_serializing_if = "Option::is_none")]
    pub application_access: Option<Vec<String>>,
    #[serde(rename = "groupCustomFields", default, skip_serializing_if = "Option::is_none")]
    pub group_custom_fields: Option<Vec<ProjectCreateResourceIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ProjectCreateResourceIdentifier>>,
    #[serde(rename = "permissionKeys", default, skip_serializing_if = "Option::is_none")]
    pub permission_keys: Option<Vec<String>>,
    #[serde(rename = "projectRoles", default, skip_serializing_if = "Option::is_none")]
    pub project_roles: Option<Vec<ProjectCreateResourceIdentifier>>,
    #[serde(rename = "specialGrants", default, skip_serializing_if = "Option::is_none")]
    pub special_grants: Option<Vec<String>>,
    #[serde(rename = "userCustomFields", default, skip_serializing_if = "Option::is_none")]
    pub user_custom_fields: Option<Vec<ProjectCreateResourceIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<ProjectCreateResourceIdentifier>>,
}
