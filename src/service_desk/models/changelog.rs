// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A log of changes made to issue fields. Changelogs related to workflow associations are currently being deprecated.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Changelog {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<UserDetails>,
    /// The date on which the change took place.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// The date on which the change took place.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub created: Option<String>,
    #[serde(rename = "historyMetadata", default, skip_serializing_if = "Option::is_none")]
    pub history_metadata: Option<HistoryMetadata>,
    /// The ID of the changelog.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The list of items changed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ChangeDetails>>,
}
