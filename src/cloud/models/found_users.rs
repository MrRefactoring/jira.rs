// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The list of users found in a search, including header text (Showing X of Y matching users) and total of matched users.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundUsers {
    /// Header text indicating the number of users in the response and the total number of users found in the search.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    /// The total number of users found in the search.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserPickerUser>>,
}
