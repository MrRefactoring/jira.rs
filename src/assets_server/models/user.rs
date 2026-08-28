// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "avatarUrl", default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(rename = "renderedLink", default, skip_serializing_if = "Option::is_none")]
    pub rendered_link: Option<String>,
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(rename = "lastSeenVersion", default, skip_serializing_if = "Option::is_none")]
    pub last_seen_version: Option<String>,
}
