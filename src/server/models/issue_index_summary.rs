// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueIndexSummary {
    #[serde(rename = "countInArchive", default, skip_serializing_if = "Option::is_none")]
    pub count_in_archive: Option<i64>,
    #[serde(rename = "countInDatabase", default, skip_serializing_if = "Option::is_none")]
    pub count_in_database: Option<i64>,
    #[serde(rename = "countInIndex", default, skip_serializing_if = "Option::is_none")]
    pub count_in_index: Option<i64>,
    #[serde(rename = "indexReadable", default, skip_serializing_if = "Option::is_none")]
    pub index_readable: Option<bool>,
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "lastUpdatedInDatabase",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub last_updated_in_database: Option<chrono::DateTime<chrono::Utc>>,
    #[cfg(not(feature = "chrono"))]
    #[serde(
        rename = "lastUpdatedInDatabase",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub last_updated_in_database: Option<String>,
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "lastUpdatedInIndex",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub last_updated_in_index: Option<chrono::DateTime<chrono::Utc>>,
    #[cfg(not(feature = "chrono"))]
    #[serde(
        rename = "lastUpdatedInIndex",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub last_updated_in_index: Option<String>,
}
