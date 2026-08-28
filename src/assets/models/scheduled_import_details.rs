// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The frequency of the scheduled import. ONCE: runs only at startTime. DAILY: runs every day at the specified time. WEEKLY: runs every 7 days. MONTHLY: runs on the same day of each month.
    pub enum ScheduledImportDetailsRunFrequency {
        Once => "ONCE",
        Daily => "DAILY",
        Weekly => "WEEKLY",
        Monthly => "MONTHLY",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ScheduledImportDetails {
    /// Schedule ID
    #[serde(rename = "importScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub import_schedule_id: Option<String>,
    /// When the schedule starts
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "startTime",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    /// When the schedule starts
    #[cfg(not(feature = "chrono"))]
    #[serde(
        rename = "startTime",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub start_time: Option<String>,
    /// The frequency of the scheduled import. ONCE: runs only at startTime. DAILY: runs every day at the specified time. WEEKLY: runs every 7 days. MONTHLY: runs on the same day of each month.
    #[serde(rename = "runFrequency", default, skip_serializing_if = "Option::is_none")]
    pub run_frequency: Option<ScheduledImportDetailsRunFrequency>,
    /// Next scheduled execution time
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "nextScheduledTime",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub next_scheduled_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Next scheduled execution time
    #[cfg(not(feature = "chrono"))]
    #[serde(
        rename = "nextScheduledTime",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub next_scheduled_time: Option<String>,
    /// When the schedule was created
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "createdAt",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// When the schedule was created
    #[cfg(not(feature = "chrono"))]
    #[serde(
        rename = "createdAt",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub created_at: Option<String>,
}
