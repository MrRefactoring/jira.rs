// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Avatar {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "avatarUUID", default, skip_serializing_if = "Option::is_none")]
    pub avatar_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url16: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url48: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url72: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url144: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url288: Option<String>,
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<i64>,
}
