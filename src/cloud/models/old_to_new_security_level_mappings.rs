// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OldToNewSecurityLevelMappings {
    /// The new issue security level ID. Providing null will clear the assigned old level from issues.
    #[serde(rename = "newLevelId")]
    pub new_level_id: String,
    /// The old issue security level ID. Providing null will remap all issues without any assigned levels.
    #[serde(rename = "oldLevelId")]
    pub old_level_id: String,
}
