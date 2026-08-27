// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JiraMultipleGroupPickerField {
    #[serde(rename = "fieldId")]
    pub field_id: String,
    pub groups: Vec<JiraGroupInput>,
}
