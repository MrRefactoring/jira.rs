// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Comment {
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub updated: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "commentOutput", default, skip_serializing_if = "Option::is_none")]
    pub comment_output: Option<String>,
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<i64>,
    #[serde(rename = "canEdit", default, skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    #[serde(rename = "canDelete", default, skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,
}
