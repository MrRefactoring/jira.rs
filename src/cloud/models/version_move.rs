// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// An absolute position in which to place the moved version. Cannot be used with `after`.
    pub enum VersionMovePosition {
        Earlier => "Earlier",
        Later => "Later",
        First => "First",
        Last => "Last",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VersionMove {
    /// The URL (self link) of the version after which to place the moved version. Cannot be used with `position`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// An absolute position in which to place the moved version. Cannot be used with `after`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<VersionMovePosition>,
}
