// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Add or clear a single select field:
///
///  *  To add, specify the option with an `optionId`.
///  *  To clear, pass an option with `optionId` as `-1`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraSingleSelectField {
    #[serde(rename = "fieldId")]
    pub field_id: String,
    pub option: JiraSelectedOptionField,
}
