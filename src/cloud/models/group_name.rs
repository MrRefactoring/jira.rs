// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details about a group.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GroupName {
    /// The ID of the group, which uniquely identifies the group across all Atlassian products. For example, *952d12c3-5b5b-4d04-bb32-44d383afc4b2*.
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The name of group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL for these group details.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
