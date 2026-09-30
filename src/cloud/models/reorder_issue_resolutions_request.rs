// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Change the order of issue resolutions.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReorderIssueResolutionsRequest {
    /// The ID of the resolution. Required if `position` isn't provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The list of resolution IDs to be reordered. Cannot contain duplicates nor after ID.
    pub ids: Vec<String>,
    /// The position for issue resolutions to be moved to. Required if `after` isn't provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}
