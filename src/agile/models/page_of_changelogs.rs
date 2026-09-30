// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A page of changelogs.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PageOfChangelogs {
    /// The list of changelogs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub histories: Option<Vec<Changelog>>,
    /// The maximum number of results that could be on the page.
    #[serde(rename = "maxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// The index of the first item returned on the page.
    #[serde(rename = "startAt", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// The number of results on the page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}
