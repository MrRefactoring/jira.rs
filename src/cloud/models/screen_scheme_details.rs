// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of a screen scheme.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScreenSchemeDetails {
    /// The description of the screen scheme. The maximum length is 255 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the screen scheme. The name must be unique. The maximum length is 255 characters.
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screens: Option<ScreenTypes>,
}
