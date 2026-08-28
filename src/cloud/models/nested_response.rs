// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NestedResponse {
    #[serde(rename = "errorCollection", default, skip_serializing_if = "Option::is_none")]
    pub error_collection: Option<ErrorCollection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    #[serde(rename = "warningCollection", default, skip_serializing_if = "Option::is_none")]
    pub warning_collection: Option<WarningCollection>,
}
