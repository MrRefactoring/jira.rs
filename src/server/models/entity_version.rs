// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EntityVersion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "entityId", default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<i64>,
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(rename = "entityVersion", default, skip_serializing_if = "Option::is_none")]
    pub entity_version: Option<i64>,
    #[serde(rename = "hasVersion", default, skip_serializing_if = "Option::is_none")]
    pub has_version: Option<bool>,
    #[serde(rename = "parentIssueId", default, skip_serializing_if = "Option::is_none")]
    pub parent_issue_id: Option<i64>,
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
    #[cfg(not(feature = "chrono"))]
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub update_time: Option<String>,
}
