// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The status of the task.
    pub enum BulkOperationProgressStatus {
        Enqueued => "ENQUEUED",
        Running => "RUNNING",
        Complete => "COMPLETE",
        Failed => "FAILED",
        CancelRequested => "CANCEL_REQUESTED",
        Cancelled => "CANCELLED",
        Dead => "DEAD",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkOperationProgress {
    /// A timestamp of when the task was submitted.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// A timestamp of when the task was submitted.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub created: Option<String>,
    /// Map of issue IDs for which the operation failed and that the user has permission to view, to their one or more reasons for failure. These reasons are open-ended text descriptions of the error and are not selected from a predefined list of standard reasons.
    #[serde(rename = "failedAccessibleIssues", default, skip_serializing_if = "Option::is_none")]
    pub failed_accessible_issues: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The number of issues that are either invalid or issues that the user doesn't have permission to view, regardless of the success or failure of the operation.
    #[serde(rename = "invalidOrInaccessibleIssueCount", default, skip_serializing_if = "Option::is_none")]
    pub invalid_or_inaccessible_issue_count: Option<i64>,
    /// List of issue IDs for which the operation was successful and that the user has permission to view.
    #[serde(rename = "processedAccessibleIssues", default, skip_serializing_if = "Option::is_none")]
    pub processed_accessible_issues: Option<Vec<i64>>,
    /// Progress of the task as a percentage.
    #[serde(rename = "progressPercent", default, skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<i64>,
    /// A timestamp of when the task was started.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub started: Option<chrono::DateTime<chrono::Utc>>,
    /// A timestamp of when the task was started.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub started: Option<String>,
    /// The status of the task.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<BulkOperationProgressStatus>,
    #[serde(rename = "submittedBy", default, skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<DashboardUser>,
    /// The ID of the task.
    #[serde(rename = "taskId", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// The number of issues that the bulk operation was attempted on.
    #[serde(rename = "totalIssueCount", default, skip_serializing_if = "Option::is_none")]
    pub total_issue_count: Option<i64>,
    /// A timestamp of when the task progress was last updated.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
    /// A timestamp of when the task progress was last updated.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub updated: Option<String>,
}
