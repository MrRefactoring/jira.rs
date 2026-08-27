// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Indicates the user information fields to include in the response. If unspecified, the response defaults to id, name and description.
    pub enum GroupsSearchRequestExpand {
        Users => "USERS",
        Meta => "META",
        RoleAssignments => "ROLE_ASSIGNMENTS",
        ManagementAccess => "MANAGEMENT_ACCESS",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupsSearchRequest {
    /// Unique ID that serves as reference to the group.
    #[serde(rename = "groupIds", default, skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    #[serde(rename = "groupNames", default, skip_serializing_if = "Option::is_none")]
    pub group_names: Option<GroupNames>,
    /// Cursor specifying the starting point for page result retrieval.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The number of items to return. Default = max = 1000
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Indicates the user information fields to include in the response. If unspecified, the response defaults to id, name and description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<GroupsSearchRequestExpand>>,
}
