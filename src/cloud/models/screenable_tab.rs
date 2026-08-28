// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A screen tab.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScreenableTab {
    /// The ID of the screen tab.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the screen tab. The maximum length is 255 characters.
    pub name: String,
}
