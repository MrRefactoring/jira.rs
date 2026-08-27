// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A page of items.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageString {
    /// Whether this is the last page.
    #[serde(rename = "isLast", default, skip_serializing_if = "Option::is_none")]
    pub is_last: Option<bool>,
    /// The maximum number of items that could be returned.
    #[serde(rename = "maxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// If there is another page of results, the URL of the next page.
    #[serde(rename = "nextPage", default, skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    /// The URL of the page.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// The index of the first item returned.
    #[serde(rename = "startAt", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// The number of items returned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// The list of items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}
