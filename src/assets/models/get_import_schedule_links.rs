// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetImportScheduleLinksLinks {
    /// URL to POST to create a new schedule
    #[serde(rename = "createSchedule")]
    pub create_schedule: String,
    /// URL to GET/PUT/DELETE an existing schedule. Only present if a schedule exists for this import source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetImportScheduleLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<GetImportScheduleLinksLinks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}
