// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SlaInformationOngoingCycle {
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Date>,
    #[serde(rename = "breachTime", default, skip_serializing_if = "Option::is_none")]
    pub breach_time: Option<Date>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breached: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(rename = "withinCalendarHours", default, skip_serializing_if = "Option::is_none")]
    pub within_calendar_hours: Option<bool>,
    #[serde(rename = "goalDuration", default, skip_serializing_if = "Option::is_none")]
    pub goal_duration: Option<Duration>,
    #[serde(rename = "elapsedTime", default, skip_serializing_if = "Option::is_none")]
    pub elapsed_time: Option<Duration>,
    #[serde(rename = "remainingTime", default, skip_serializing_if = "Option::is_none")]
    pub remaining_time: Option<Duration>,
}
