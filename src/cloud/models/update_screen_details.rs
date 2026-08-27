// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of a screen.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateScreenDetails {
    /// The description of the screen. The maximum length is 255 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the screen. The name must be unique. The maximum length is 255 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
