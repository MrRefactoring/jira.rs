// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A page containing dashboard details.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageOfDashboards {
    /// List of dashboards.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<Vec<Dashboard>>,
    /// The maximum number of results that could be on the page.
    #[serde(rename = "maxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// The URL of the next page of results, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    /// The URL of the previous page of results, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    /// The index of the first item returned on the page.
    #[serde(rename = "startAt", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// The number of results on the page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}
