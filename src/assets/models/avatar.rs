// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The object avatar is a custom image that represents an object. If the object has no avatar the icon for the object type will be used
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Avatar {
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[serde(rename = "globalId")]
    pub global_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "avatarUUID", default, skip_serializing_if = "Option::is_none")]
    pub avatar_uuid: Option<String>,
    pub url16: String,
    pub url48: String,
    pub url72: String,
    pub url144: String,
    pub url288: String,
    /// A reference to the object that this avatar is associated with
    #[serde(rename = "objectId")]
    pub object_id: String,
}
