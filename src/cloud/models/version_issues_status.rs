// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Counts of the number of issues in various statuses.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VersionIssuesStatus {
    /// Count of issues with status *done*.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub done: Option<i64>,
    /// Count of issues with status *in progress*.
    #[serde(rename = "inProgress", default, skip_serializing_if = "Option::is_none")]
    pub in_progress: Option<i64>,
    /// Count of issues with status *to do*.
    #[serde(rename = "toDo", default, skip_serializing_if = "Option::is_none")]
    pub to_do: Option<i64>,
    /// Count of issues with a status other than *to do*, *in progress*, and *done*.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unmapped: Option<i64>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}
