// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of an issue remote link.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RemoteIssueLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<Application>,
    /// The global ID of the link, such as the ID of the item on the remote system.
    #[serde(rename = "globalId", default, skip_serializing_if = "Option::is_none")]
    pub global_id: Option<String>,
    /// The ID of the link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<RemoteObject>,
    /// Description of the relationship between the issue and the linked item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
    /// The URL of the link.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
