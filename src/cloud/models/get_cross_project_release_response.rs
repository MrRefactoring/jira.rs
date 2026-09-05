// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetCrossProjectReleaseResponse {
    /// The cross-project release name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The IDs of the releases included in the cross-project release.
    #[serde(rename = "releaseIds", default, skip_serializing_if = "Option::is_none")]
    pub release_ids: Option<Vec<i64>>,
}
