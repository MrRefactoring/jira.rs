// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum VersionMovePosition {
        Earlier => "Earlier",
        Later => "Later",
        First => "First",
        Last => "Last",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VersionMove {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<VersionMovePosition>,
}
