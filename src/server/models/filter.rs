// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Filter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favourite: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jql: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<User>,
    #[serde(rename = "searchUrl", default, skip_serializing_if = "Option::is_none")]
    pub search_url: Option<String>,
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "sharePermissions", default, skip_serializing_if = "Option::is_none")]
    pub share_permissions: Option<Vec<FilterPermission>>,
    #[serde(rename = "sharedUsers", default, skip_serializing_if = "Option::is_none")]
    pub shared_users: Option<UserListWrapper>,
    #[serde(rename = "viewUrl", default, skip_serializing_if = "Option::is_none")]
    pub view_url: Option<String>,
}
