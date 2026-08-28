// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The payload for the layout details for the start end of a transition
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FromLayoutPayload {
    /// The port that the transition can be made from
    #[serde(rename = "fromPort", default, skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ProjectCreateResourceIdentifier>,
    /// The port that the transition goes to
    #[serde(rename = "toPortOverride", default, skip_serializing_if = "Option::is_none")]
    pub to_port_override: Option<i64>,
}
