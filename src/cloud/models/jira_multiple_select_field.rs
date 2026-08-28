// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraMultipleSelectField {
    #[serde(rename = "fieldId")]
    pub field_id: String,
    pub options: Vec<JiraSelectedOptionField>,
}
