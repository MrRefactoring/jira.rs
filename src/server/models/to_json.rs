// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupJson>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reporter: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserJson>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voters: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watchers: Option<bool>,
}
