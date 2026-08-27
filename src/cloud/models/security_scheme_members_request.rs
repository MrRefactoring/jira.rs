// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of issue security scheme level new members.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecuritySchemeMembersRequest {
    /// The list of level members which should be added to the issue security scheme level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<SecuritySchemeLevelMember>>,
}
