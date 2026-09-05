// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of changes to a priority scheme that require suggested priority mappings.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SuggestedMappingsRequest {
    /// The maximum number of results that could be on the page.
    #[serde(rename = "maxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priorities: Option<SuggestedMappingsForPrioritiesRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<SuggestedMappingsForProjectsRequest>,
    /// The id of the priority scheme.
    #[serde(rename = "schemeId", default, skip_serializing_if = "Option::is_none")]
    pub scheme_id: Option<i64>,
    /// The index of the first item returned on the page.
    #[serde(rename = "startAt", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
}
