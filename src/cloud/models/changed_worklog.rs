// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of a changed worklog.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangedWorklog {
    /// Details of properties associated with the change.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<EntityProperty>>,
    /// The datetime of the change.
    #[serde(rename = "updatedTime", default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<i64>,
    /// The ID of the worklog.
    #[serde(rename = "worklogId", default, skip_serializing_if = "Option::is_none")]
    pub worklog_id: Option<i64>,
}
