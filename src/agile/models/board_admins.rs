// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The users and groups who own the board.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BoardAdmins {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}
