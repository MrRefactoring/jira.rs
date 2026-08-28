// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JiraCascadingSelectField {
    #[serde(rename = "childOptionValue", default, skip_serializing_if = "Option::is_none")]
    pub child_option_value: Option<JiraSelectedOptionField>,
    #[serde(rename = "fieldId")]
    pub field_id: String,
    #[serde(rename = "parentOptionValue")]
    pub parent_option_value: JiraSelectedOptionField,
}
