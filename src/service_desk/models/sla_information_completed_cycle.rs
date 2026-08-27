// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SlaInformationCompletedCycle {
    #[serde(rename = "breachTime", default, skip_serializing_if = "Option::is_none")]
    pub breach_time: Option<Date>,
    /// Indicates if the SLA (duration) was exceeded (true) or not (false).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breached: Option<bool>,
    #[serde(rename = "elapsedTime", default, skip_serializing_if = "Option::is_none")]
    pub elapsed_time: Option<Duration>,
    #[serde(rename = "goalDuration", default, skip_serializing_if = "Option::is_none")]
    pub goal_duration: Option<Duration>,
    #[serde(rename = "remainingTime", default, skip_serializing_if = "Option::is_none")]
    pub remaining_time: Option<Duration>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Date>,
    #[serde(rename = "stopTime", default, skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<Date>,
}
