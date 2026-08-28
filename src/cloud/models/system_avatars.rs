// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// List of system avatars.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SystemAvatars {
    /// A list of avatar details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<Vec<Avatar>>,
}
