// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JiraSingleGroupPickerField {
    #[serde(rename = "fieldId")]
    pub field_id: String,
    pub group: JiraGroupInput,
}
