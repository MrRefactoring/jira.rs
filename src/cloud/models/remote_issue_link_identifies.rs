// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of the identifiers for a created or updated remote issue link.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RemoteIssueLinkIdentifies {
    /// The ID of the remote issue link, such as the ID of the item on the remote system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The URL of the remote issue link.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
