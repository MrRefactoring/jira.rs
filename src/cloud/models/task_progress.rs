// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The status of the task.
    pub enum TaskProgressStatus {
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
pub struct TaskProgress {
    /// The description of the task.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The execution time of the task, in milliseconds.
    #[serde(rename = "elapsedRuntime")]
    pub elapsed_runtime: i64,
    /// A timestamp recording when the task was finished.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub finished: Option<chrono::DateTime<chrono::Utc>>,
    /// A timestamp recording when the task was finished.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub finished: Option<String>,
    /// The ID of the task.
    pub id: String,
    /// A timestamp recording when the task progress was last updated.
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "lastUpdate",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub last_update: Option<chrono::DateTime<chrono::Utc>>,
    /// A timestamp recording when the task progress was last updated.
    #[cfg(not(feature = "chrono"))]
    #[serde(rename = "lastUpdate", deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub last_update: String,
    /// Information about the progress of the task.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The progress of the task, as a percentage complete.
    pub progress: i64,
    /// The result of the task execution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// The URL of the task.
    #[serde(rename = "self")]
    pub self_: String,
    /// A timestamp recording when the task was started.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub started: Option<chrono::DateTime<chrono::Utc>>,
    /// A timestamp recording when the task was started.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub started: Option<String>,
    /// The status of the task.
    pub status: TaskProgressStatus,
    /// A timestamp recording when the task was submitted.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub submitted: Option<chrono::DateTime<chrono::Utc>>,
    /// A timestamp recording when the task was submitted.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub submitted: Option<String>,
    /// The ID of the user who submitted the task.
    #[serde(rename = "submittedBy")]
    pub submitted_by: i64,
}
