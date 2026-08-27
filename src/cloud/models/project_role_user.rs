// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of the user associated with the role.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectRoleUser {
    /// The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*. Returns *unknown* if the record is deleted and corrupted, for example, as the result of a server import.
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}
