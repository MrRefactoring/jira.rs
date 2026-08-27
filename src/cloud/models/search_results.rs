// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The result of a JQL search.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchResults {
    /// Expand options that include additional search result details in the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// The list of issues found by the search.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<Issue>>,
    /// The maximum number of results that could be on the page.
    #[serde(rename = "maxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// The ID and name of each field in the search results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The schema describing the field types in the search results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The index of the first item returned on the page.
    #[serde(rename = "startAt", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// The number of results on the page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Any warnings related to the JQL query.
    #[serde(rename = "warningMessages", default, skip_serializing_if = "Option::is_none")]
    pub warning_messages: Option<Vec<String>>,
}
