// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The frequency of the scheduled import. ONCE: runs only at startTime. DAILY: runs every day at the specified time. WEEKLY: runs every 7 days. MONTHLY: runs on the same day of each month.
    pub enum ImportScheduleRequestRunInterval {
        Once => "ONCE",
        Daily => "DAILY",
        Weekly => "WEEKLY",
        Monthly => "MONTHLY",
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportScheduleRequest {
    /// The date and time when the first import should execute, in ISO 8601 format (e.g., '2024-01-15T02:00:00Z'). Must be in the future.
    #[serde(rename = "startTime", deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub start_time: String,
    /// The frequency of the scheduled import. ONCE: runs only at startTime. DAILY: runs every day at the specified time. WEEKLY: runs every 7 days. MONTHLY: runs on the same day of each month.
    #[serde(rename = "runInterval")]
    pub run_interval: ImportScheduleRequestRunInterval,
    /// Optional webhook URL to call after each scheduled import execution. The URL will receive a POST request with execution status.
    #[serde(rename = "callbackUrl", default, skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
}
