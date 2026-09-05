// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SoftwareNavigationInfo {
    #[serde(rename = "boardId", default, skip_serializing_if = "Option::is_none")]
    pub board_id: Option<i64>,
    #[serde(rename = "boardName", default, skip_serializing_if = "Option::is_none")]
    pub board_name: Option<String>,
    #[serde(rename = "simpleBoard", default, skip_serializing_if = "Option::is_none")]
    pub simple_board: Option<bool>,
    #[serde(rename = "totalBoardsInProject", default, skip_serializing_if = "Option::is_none")]
    pub total_boards_in_project: Option<i64>,
}
