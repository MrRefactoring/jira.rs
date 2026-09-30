// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Version {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "moveUnfixedIssuesTo", default, skip_serializing_if = "Option::is_none")]
    pub move_unfixed_issues_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overdue: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "releaseDate",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub release_date: Option<chrono::DateTime<chrono::Utc>>,
    #[cfg(not(feature = "chrono"))]
    #[serde(
        rename = "releaseDate",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub release_date: Option<String>,
    #[serde(rename = "releaseDateSet", default, skip_serializing_if = "Option::is_none")]
    pub release_date_set: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub released: Option<bool>,
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "startDate",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    #[cfg(not(feature = "chrono"))]
    #[serde(
        rename = "startDate",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub start_date: Option<String>,
    #[serde(rename = "startDateSet", default, skip_serializing_if = "Option::is_none")]
    pub start_date_set: Option<bool>,
    #[serde(rename = "userReleaseDate", default, skip_serializing_if = "Option::is_none")]
    pub user_release_date: Option<String>,
    #[serde(rename = "userStartDate", default, skip_serializing_if = "Option::is_none")]
    pub user_start_date: Option<String>,
}
