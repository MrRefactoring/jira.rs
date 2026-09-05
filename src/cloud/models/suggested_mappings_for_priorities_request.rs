// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of changes to a priority scheme's priorities that require suggested priority mappings.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SuggestedMappingsForPrioritiesRequest {
    /// The ids of priorities being removed from the scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<i64>>,
    /// The ids of priorities being removed from the scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<i64>>,
}
