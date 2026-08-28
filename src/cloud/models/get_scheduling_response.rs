// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The dependencies for the plan. This is "Sequential" or "Concurrent".
    pub enum GetSchedulingResponseDependencies {
        Sequential => "Sequential",
        Concurrent => "Concurrent",
    }
}

crate::open_enum! {
    /// The estimation unit for the plan. This is "StoryPoints", "Days" or "Hours".
    pub enum GetSchedulingResponseEstimation {
        StoryPoints => "StoryPoints",
        Days => "Days",
        Hours => "Hours",
    }
}

crate::open_enum! {
    /// The inferred dates for the plan. This is "None", "SprintDates" or "ReleaseDates".
    pub enum GetSchedulingResponseInferredDates {
        None => "None",
        SprintDates => "SprintDates",
        ReleaseDates => "ReleaseDates",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetSchedulingResponse {
    /// The dependencies for the plan. This is "Sequential" or "Concurrent".
    pub dependencies: GetSchedulingResponseDependencies,
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<GetDateFieldResponse>,
    /// The estimation unit for the plan. This is "StoryPoints", "Days" or "Hours".
    pub estimation: GetSchedulingResponseEstimation,
    /// The inferred dates for the plan. This is "None", "SprintDates" or "ReleaseDates".
    #[serde(rename = "inferredDates")]
    pub inferred_dates: GetSchedulingResponseInferredDates,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<GetDateFieldResponse>,
}
