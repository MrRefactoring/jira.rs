// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JiraColorField {
    pub color: JiraColorInput,
    #[serde(rename = "fieldId")]
    pub field_id: String,
}
