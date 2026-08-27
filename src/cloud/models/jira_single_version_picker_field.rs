// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraSingleVersionPickerField {
    #[serde(rename = "fieldId")]
    pub field_id: String,
    pub version: JiraVersionField,
}
