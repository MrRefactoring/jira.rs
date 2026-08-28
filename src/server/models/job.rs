// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Job {
    #[serde(rename = "cronExpression", default, skip_serializing_if = "Option::is_none")]
    pub cron_expression: Option<String>,
    #[serde(rename = "firstRunTime", default, skip_serializing_if = "Option::is_none")]
    pub first_run_time: Option<i64>,
    #[serde(rename = "intervalInMillis", default, skip_serializing_if = "Option::is_none")]
    pub interval_in_millis: Option<i64>,
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobRunnerKey", default, skip_serializing_if = "Option::is_none")]
    pub job_runner_key: Option<String>,
    #[serde(rename = "nextRunTime", default, skip_serializing_if = "Option::is_none")]
    pub next_run_time: Option<i64>,
    #[serde(rename = "runMode", default, skip_serializing_if = "Option::is_none")]
    pub run_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runnable: Option<bool>,
    #[serde(rename = "scheduleType", default, skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<String>,
    #[serde(rename = "timeZoneId", default, skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
}
