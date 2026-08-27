// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The named position to which the screen tab field should be moved. Required if `after` isn't provided.
    pub enum MoveFieldPosition {
        Earlier => "Earlier",
        Later => "Later",
        First => "First",
        Last => "Last",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MoveField {
    /// The ID of the screen tab field after which to place the moved screen tab field. Required if `position` isn't provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The named position to which the screen tab field should be moved. Required if `after` isn't provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<MoveFieldPosition>,
}
