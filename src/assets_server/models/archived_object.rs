// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// An object that has been archived, as the archive listing describes it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArchivedObject {
    pub id: i64,
    pub key: String,
    pub label: String,
    #[serde(rename = "hasAvatar", default, skip_serializing_if = "Option::is_none")]
    pub has_avatar: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Avatar>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<ObjectType>,
    #[serde(rename = "objectSchema", default, skip_serializing_if = "Option::is_none")]
    pub object_schema: Option<ObjectSchema>,
    pub archived: bool,
    #[serde(rename = "archivedDate", default, skip_serializing_if = "Option::is_none")]
    pub archived_date: Option<String>,
    #[serde(rename = "archivedBy", default, skip_serializing_if = "Option::is_none")]
    pub archived_by: Option<User>,
}
