// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The frequency of the scheduled import
    pub enum ImportScheduleResponseRunInterval {
        Once => "ONCE",
        Daily => "DAILY",
        Weekly => "WEEKLY",
        Monthly => "MONTHLY",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportScheduleResponse {
    /// The unique identifier of the import schedule
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID of the associated import source
    #[serde(rename = "importSourceId", default, skip_serializing_if = "Option::is_none")]
    pub import_source_id: Option<String>,
    /// The start time of the schedule in ISO 8601 format
    #[serde(
        rename = "startTime",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub start_time: Option<String>,
    /// The frequency of the scheduled import
    #[serde(rename = "runInterval", default, skip_serializing_if = "Option::is_none")]
    pub run_interval: Option<ImportScheduleResponseRunInterval>,
    /// Timestamp when the schedule was created
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub created: Option<String>,
    /// Timestamp when the schedule was last updated
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub updated: Option<String>,
    /// The collection (object schema) ID associated with this import
    #[serde(rename = "collectionId", default, skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
}
