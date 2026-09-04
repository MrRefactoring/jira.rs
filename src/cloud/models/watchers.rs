// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The details of watchers on an issue.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Watchers {
    /// Whether the calling user is watching this issue.
    #[serde(rename = "isWatching", default, skip_serializing_if = "Option::is_none")]
    pub is_watching: Option<bool>,
    /// The URL of these issue watcher details.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// The number of users watching this issue.
    #[serde(rename = "watchCount", default, skip_serializing_if = "Option::is_none")]
    pub watch_count: Option<i64>,
    /// Details of the users watching this issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watchers: Option<Vec<UserDetails>>,
}
