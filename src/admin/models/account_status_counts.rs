// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountStatusCounts {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AccountStatus>,
    /// The number of accounts with the associated status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
