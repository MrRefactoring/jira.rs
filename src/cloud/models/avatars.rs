// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details about system and custom avatars.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Avatars {
    /// Custom avatars list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Vec<Avatar>>,
    /// System avatars list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<Vec<Avatar>>,
}
