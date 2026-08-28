// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkManagementNavigationInfo {
    #[serde(rename = "boardName", default, skip_serializing_if = "Option::is_none")]
    pub board_name: Option<String>,
}
