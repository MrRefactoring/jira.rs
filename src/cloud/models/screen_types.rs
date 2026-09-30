// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The IDs of the screens for the screen types of the screen scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScreenTypes {
    /// The ID of the create screen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<i64>,
    /// The ID of the default screen. Required when creating a screen scheme.
    pub default: i64,
    /// The ID of the edit screen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edit: Option<i64>,
    /// The ID of the view screen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<i64>,
}
