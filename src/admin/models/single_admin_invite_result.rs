// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SingleAdminInviteResult {
    /// List of role assignment results for the user
    #[serde(rename = "roleAssignmentResult", default, skip_serializing_if = "Option::is_none")]
    pub role_assignment_result: Option<Vec<RoleAssignmentResult>>,
    /// List of group assignment results for the user
    #[serde(rename = "groupAssignmentResult", default, skip_serializing_if = "Option::is_none")]
    pub group_assignment_result: Option<Vec<GroupAssignmentResult>>,
}
