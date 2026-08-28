// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Property key details.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PropertyKey {
    /// The key of the property.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The URL of the property.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
