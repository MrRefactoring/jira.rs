// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The status of the task.
    pub enum TaskProgressRemoveOptionFromIssuesResultStatus {
        Enqueued => "ENQUEUED",
        Running => "RUNNING",
        Complete => "COMPLETE",
        Failed => "FAILED",
        CancelRequested => "CANCEL_REQUESTED",
        Cancelled => "CANCELLED",
        Dead => "DEAD",
    }
}

/// Details about a task.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TaskProgressRemoveOptionFromIssuesResult {
    /// The description of the task.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The execution time of the task, in milliseconds.
    #[serde(rename = "elapsedRuntime")]
    pub elapsed_runtime: i64,
    /// A timestamp recording when the task was finished.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished: Option<i64>,
    /// The ID of the task.
    pub id: String,
    /// A timestamp recording when the task progress was last updated.
    #[serde(rename = "lastUpdate")]
    pub last_update: i64,
    /// Information about the progress of the task.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The progress of the task, as a percentage complete.
    pub progress: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<RemoveOptionFromIssuesResult>,
    /// The URL of the task.
    #[serde(rename = "self")]
    pub self_: String,
    /// A timestamp recording when the task was started.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<i64>,
    /// The status of the task.
    pub status: TaskProgressRemoveOptionFromIssuesResultStatus,
    /// A timestamp recording when the task was submitted.
    pub submitted: i64,
    /// The ID of the user who submitted the task.
    #[serde(rename = "submittedBy")]
    pub submitted_by: i64,
}
