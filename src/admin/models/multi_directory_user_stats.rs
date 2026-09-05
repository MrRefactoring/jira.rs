// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MultiDirectoryUserStats {
    /// User counts associated with different role IDs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RoleIdCounts>>,
    /// User counts associated with different account statuses.
    #[serde(rename = "accountStatus", default, skip_serializing_if = "Option::is_none")]
    pub account_status: Option<Vec<AccountStatusCounts>>,
}
