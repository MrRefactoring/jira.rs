// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkOperationErrorResult {
    #[serde(rename = "elementErrors", default, skip_serializing_if = "Option::is_none")]
    pub element_errors: Option<ErrorCollection>,
    #[serde(rename = "failedElementNumber", default, skip_serializing_if = "Option::is_none")]
    pub failed_element_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}
