// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The result of a JQL search with issues reconsilation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchAndReconcileResults {
    /// Indicates whether this is the last page of the paginated response.
    #[serde(rename = "isLast", default, skip_serializing_if = "Option::is_none")]
    pub is_last: Option<bool>,
    /// The list of issues found by the search or reconsiliation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<Issue>>,
    /// The ID and name of each field in the search results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Continuation token to fetch the next page. If this result represents the last or the only page this token will be null. This token will expire in 7 days.
    #[serde(rename = "nextPageToken", default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// The schema describing the field types in the search results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Experimental. Warnings generated during the search, e.g. when a JQL clause exceeded its argument limit or when the result set was truncated due to an ingestion limit. This field is currently rolling out behind a feature flag and may be absent, empty, or change shape without notice until generally available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<SearchWarning>>,
}
