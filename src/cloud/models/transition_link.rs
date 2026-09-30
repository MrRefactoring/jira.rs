// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Link information for workflow transitions.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TransitionLink {
    /// The from port number.
    #[serde(rename = "fromPort", default, skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i64>,
    /// The from status reference.
    #[serde(rename = "fromStatusReference", default, skip_serializing_if = "Option::is_none")]
    pub from_status_reference: Option<String>,
    /// The to port number.
    #[serde(rename = "toPort", default, skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i64>,
}
