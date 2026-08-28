// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isAllProjects", default, skip_serializing_if = "Option::is_none")]
    pub is_all_projects: Option<bool>,
    #[serde(rename = "isLocked", default, skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(rename = "isManaged", default, skip_serializing_if = "Option::is_none")]
    pub is_managed: Option<bool>,
    #[serde(rename = "isTrusted", default, skip_serializing_if = "Option::is_none")]
    pub is_trusted: Option<bool>,
    #[serde(rename = "issueTypeIds", default, skip_serializing_if = "Option::is_none")]
    pub issue_type_ids: Option<Vec<String>>,
    #[serde(rename = "issuesWithValue", default, skip_serializing_if = "Option::is_none")]
    pub issues_with_value: Option<i64>,
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "lastValueUpdate",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub last_value_update: Option<chrono::DateTime<chrono::Utc>>,
    #[cfg(not(feature = "chrono"))]
    #[serde(
        rename = "lastValueUpdate",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub last_value_update: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "numericId", default, skip_serializing_if = "Option::is_none")]
    pub numeric_id: Option<i64>,
    #[serde(rename = "projectIds", default, skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<i64>>,
    #[serde(rename = "projectsCount", default, skip_serializing_if = "Option::is_none")]
    pub projects_count: Option<i64>,
    #[serde(rename = "screensCount", default, skip_serializing_if = "Option::is_none")]
    pub screens_count: Option<i64>,
    #[serde(rename = "searcherKey", default, skip_serializing_if = "Option::is_none")]
    pub searcher_key: Option<String>,
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
