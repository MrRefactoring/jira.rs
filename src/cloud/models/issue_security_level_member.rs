// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Issue security level member.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueSecurityLevelMember {
    pub holder: PermissionHolder,
    /// The ID of the issue security level member.
    pub id: i64,
    /// The ID of the issue security level.
    #[serde(rename = "issueSecurityLevelId")]
    pub issue_security_level_id: i64,
}
