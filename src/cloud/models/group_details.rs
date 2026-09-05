// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details about a group.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GroupDetails {
    /// The ID of the group, which uniquely identifies the group across all Atlassian products. For example, *952d12c3-5b5b-4d04-bb32-44d383afc4b2*.
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The name of the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
