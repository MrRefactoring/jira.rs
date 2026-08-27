// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A priority scheme with less fields to be used in for an API expand response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExpandPriorityScheme {
    /// The ID of the priority scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the priority scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the priority scheme.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
