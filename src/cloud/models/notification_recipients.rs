// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of the users and groups to receive the notification.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationRecipients {
    /// Whether the notification should be sent to the issue's assignees.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<bool>,
    /// List of groupIds to receive the notification.
    #[serde(rename = "groupIds", default, skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// List of groups to receive the notification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupName>>,
    /// Whether the notification should be sent to the issue's reporter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reporter: Option<bool>,
    /// List of users to receive the notification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserDetails>>,
    /// Whether the notification should be sent to the issue's voters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voters: Option<bool>,
    /// Whether the notification should be sent to the issue's watchers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watchers: Option<bool>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}
