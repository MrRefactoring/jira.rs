// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A role assigned on a resource, including how the role was assigned.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MultiDirectoryRoleAssignment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<RoleId>,
    /// The methods by which this role was assigned to the user for the resource.
    #[serde(rename = "roleAssignmentMethods", default, skip_serializing_if = "Option::is_none")]
    pub role_assignment_methods: Option<Vec<MultiDirectoryRoleAssignmentMethod>>,
}
