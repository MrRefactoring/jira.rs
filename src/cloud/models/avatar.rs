// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of an avatar.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Avatar {
    /// The file name of the avatar icon. Returned for system avatars.
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// The ID of the avatar.
    pub id: String,
    /// Whether the avatar can be deleted.
    #[serde(rename = "isDeletable", default, skip_serializing_if = "Option::is_none")]
    pub is_deletable: Option<bool>,
    /// Whether the avatar is used in Jira. For example, shown as a project's avatar.
    #[serde(rename = "isSelected", default, skip_serializing_if = "Option::is_none")]
    pub is_selected: Option<bool>,
    /// Whether the avatar is a system avatar.
    #[serde(rename = "isSystemAvatar", default, skip_serializing_if = "Option::is_none")]
    pub is_system_avatar: Option<bool>,
    /// The owner of the avatar. For a system avatar the owner is null (and nothing is returned). For non-system avatars this is the appropriate identifier, such as the ID for a project or the account ID for a user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// The list of avatar icon URLs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urls: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}
