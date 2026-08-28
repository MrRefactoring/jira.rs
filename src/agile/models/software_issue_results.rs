// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The result of an issue search in Jira Software APIs.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SoftwareIssueResults {
    /// Expand options that include additional search result details in the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// Indicates whether this is the last page of the paginated response.
    #[serde(rename = "isLast")]
    pub is_last: bool,
    /// The list of issues found by the search.
    pub issues: Vec<Issue>,
    /// The ID and name of each field in the search results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Continuation token to fetch the next page. If this result represents the last or only page, this token will be null.
    #[serde(rename = "nextPageToken", default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// The schema describing the field types in the search results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Any warnings related to the JQL query.
    #[serde(rename = "warningMessages", default, skip_serializing_if = "Option::is_none")]
    pub warning_messages: Option<Vec<String>>,
}
