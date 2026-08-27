// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum TimeTrackingConfigurationDefaultUnit {
        Minute => "minute",
        Hour => "hour",
        Day => "day",
        Week => "week",
    }
}

crate::open_enum! {
    pub enum TimeTrackingConfigurationTimeFormat {
        Pretty => "pretty",
        Days => "days",
        Hours => "hours",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeTrackingConfiguration {
    #[serde(rename = "defaultUnit", default, skip_serializing_if = "Option::is_none")]
    pub default_unit: Option<TimeTrackingConfigurationDefaultUnit>,
    #[serde(rename = "timeFormat", default, skip_serializing_if = "Option::is_none")]
    pub time_format: Option<TimeTrackingConfigurationTimeFormat>,
    #[serde(rename = "workingDaysPerWeek", default, skip_serializing_if = "Option::is_none")]
    pub working_days_per_week: Option<f64>,
    #[serde(rename = "workingHoursPerDay", default, skip_serializing_if = "Option::is_none")]
    pub working_hours_per_day: Option<f64>,
}
