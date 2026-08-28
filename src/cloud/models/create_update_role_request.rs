// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateUpdateRoleRequest {
    /// A description of the project role. Required when fully updating a project role. Optional when creating or partially updating a project role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the project role. Must be unique. Cannot begin or end with whitespace. The maximum length is 255 characters. Required when creating a project role. Optional when partially updating a project role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
