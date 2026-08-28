// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of changes to a priority scheme's projects that require suggested priority mappings.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SuggestedMappingsForProjectsRequest {
    /// The ids of projects being added to the scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<i64>>,
}
