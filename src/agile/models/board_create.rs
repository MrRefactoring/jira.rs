// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum BoardCreateType {
        Kanban => "kanban",
        Scrum => "scrum",
        Agility => "agility",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BoardCreate {
    #[serde(rename = "filterId", default, skip_serializing_if = "Option::is_none")]
    pub filter_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<BoardCreateType>,
}
