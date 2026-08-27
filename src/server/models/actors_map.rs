// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorsMap {
    /// The usernames to add to the role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<Vec<String>>,
    /// The groups to add to the role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Vec<String>>,
}
