// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FoundUsersAndGroups {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<FoundGroups>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<FoundUsers>,
}
