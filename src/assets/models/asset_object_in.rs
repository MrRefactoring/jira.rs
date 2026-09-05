// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Representing an object to be created or updated
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetObjectIn {
    /// The object type determines where the object should be stored and which attributes are available
    #[serde(rename = "objectTypeId")]
    pub object_type_id: String,
    pub attributes: Vec<ObjectAttributeIn>,
    #[serde(rename = "hasAvatar", default, skip_serializing_if = "Option::is_none")]
    pub has_avatar: Option<bool>,
    /// The UUID as retrieved by uploading an avatar.
    #[serde(rename = "avatarUUID", default, skip_serializing_if = "Option::is_none")]
    pub avatar_uuid: Option<String>,
}
