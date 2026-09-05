// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The payload for the layout details for the destination end of a transition
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ToLayoutPayload {
    /// Defines where the transition line will be connected to a status. Port 0 to 7 are acceptable values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ProjectCreateResourceIdentifier>,
}
