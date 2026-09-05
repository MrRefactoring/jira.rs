// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MultiDirectoryUserRoleAssignmentPage {
    /// A page of user role assignments.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<MultiDirectoryUserRoleAssignment>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<LinkPageCursor>,
}
