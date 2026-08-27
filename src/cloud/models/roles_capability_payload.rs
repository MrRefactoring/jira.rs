// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RolesCapabilityPayload {
    /// A map of role PCRI (can be ID or REF) to a list of user or group PCRI IDs to associate with the role and project.
    #[serde(rename = "roleToProjectActors", default, skip_serializing_if = "Option::is_none")]
    pub role_to_project_actors: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The list of roles to create.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RolePayload>>,
}
