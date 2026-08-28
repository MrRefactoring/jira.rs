// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AdminInviteResponse {
    /// The account ID of the invited user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The email address of the invited user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// List of role and group assignment results for each user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<SingleAdminInviteResult>>,
}
