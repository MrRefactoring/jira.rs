// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The type the board.
    pub enum BoardType {
        Scrum => "scrum",
        Kanban => "kanban",
        Simple => "simple",
    }
}

/// Details about a board.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Board {
    /// The users and groups who own the board.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admins: Option<BoardAdmins>,
    /// Whether the board can be edited.
    #[serde(rename = "canEdit", default, skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    /// Whether the board is selected as a favorite.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favourite: Option<bool>,
    /// The ID of the board.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Whether the board is private.
    #[serde(rename = "isPrivate", default, skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    /// The container that the board is located in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<BoardLocation>,
    /// The name of the board.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the board.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// The type the board.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<BoardType>,
}
