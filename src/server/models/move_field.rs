// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum MoveFieldPosition {
        Earlier => "Earlier",
        Later => "Later",
        First => "First",
        Last => "Last",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MoveField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<MoveFieldPosition>,
}
