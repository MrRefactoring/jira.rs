// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorklogChange {
    #[serde(rename = "updatedTime", default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<i64>,
    #[serde(rename = "worklogId", default, skip_serializing_if = "Option::is_none")]
    pub worklog_id: Option<i64>,
}
