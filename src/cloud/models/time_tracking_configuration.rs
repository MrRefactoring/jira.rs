// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The default unit of time applied to logged time.
    pub enum TimeTrackingConfigurationDefaultUnit {
        Minute => "minute",
        Hour => "hour",
        Day => "day",
        Week => "week",
    }
}

crate::open_enum! {
    /// The format that will appear on an issue's *Time Spent* field.
    pub enum TimeTrackingConfigurationTimeFormat {
        Pretty => "pretty",
        Days => "days",
        Hours => "hours",
    }
}

/// Details of the time tracking configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTrackingConfiguration {
    /// The default unit of time applied to logged time.
    #[serde(rename = "defaultUnit")]
    pub default_unit: TimeTrackingConfigurationDefaultUnit,
    /// The format that will appear on an issue's *Time Spent* field.
    #[serde(rename = "timeFormat")]
    pub time_format: TimeTrackingConfigurationTimeFormat,
    /// The number of days in a working week.
    #[serde(rename = "workingDaysPerWeek")]
    pub working_days_per_week: f64,
    /// The number of hours in a working day.
    #[serde(rename = "workingHoursPerDay")]
    pub working_hours_per_day: f64,
}
