// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Avatar {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    #[serde(rename = "isSelected", default, skip_serializing_if = "Option::is_none")]
    pub is_selected: Option<bool>,
    #[serde(rename = "isSystemAvatar", default, skip_serializing_if = "Option::is_none")]
    pub is_system_avatar: Option<bool>,
    #[serde(rename = "isDeletable", default, skip_serializing_if = "Option::is_none")]
    pub is_deletable: Option<bool>,
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// The avatar at each size, keyed by size.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urls: Option<std::collections::HashMap<String, serde_json::Value>>,
}
