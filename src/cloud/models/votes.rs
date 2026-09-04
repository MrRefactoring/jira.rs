// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The details of votes on an issue.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Votes {
    /// Whether the user making this request has voted on the issue.
    #[serde(rename = "hasVoted", default, skip_serializing_if = "Option::is_none")]
    pub has_voted: Option<bool>,
    /// The URL of these issue vote details.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// List of the users who have voted on this issue. An empty list is returned when the calling user doesn't have the *View voters and watchers* project permission.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voters: Option<Vec<DashboardUser>>,
    /// The number of votes on the issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub votes: Option<i64>,
}
