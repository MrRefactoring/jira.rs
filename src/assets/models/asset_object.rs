// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AssetObjectLinks {
    #[serde(rename = "self")]
    pub self_: String,
}

/// An Assets object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AssetObject {
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[serde(rename = "globalId")]
    pub global_id: String,
    pub id: String,
    /// The name of the object. This value is fetched from the attribute that is currently marked as label for the object type of this object
    pub label: String,
    /// The external identifier for this object
    #[serde(rename = "objectKey")]
    pub object_key: String,
    pub avatar: Avatar,
    #[serde(rename = "objectType")]
    pub object_type: ObjectType,
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[cfg(not(feature = "chrono"))]
    #[serde(deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub created: String,
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
    #[cfg(not(feature = "chrono"))]
    #[serde(deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub updated: String,
    #[serde(rename = "hasAvatar")]
    pub has_avatar: bool,
    pub timestamp: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<ObjectAttribute>>,
    #[serde(rename = "_links")]
    pub links: AssetObjectLinks,
}
