// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Request bean for bulk changelog retrieval
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BulkChangelogRequest {
    /// List of field IDs to filter changelogs
    #[serde(rename = "fieldIds", default, skip_serializing_if = "Option::is_none")]
    pub field_ids: Option<Vec<String>>,
    /// List of issue IDs/keys to fetch changelogs for
    #[serde(rename = "issueIdsOrKeys")]
    pub issue_ids_or_keys: Vec<String>,
    /// The maximum number of items to return per page
    #[serde(rename = "maxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// The cursor for pagination
    #[serde(rename = "nextPageToken", default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}
