// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraSingleLineTextField {
    #[serde(rename = "fieldId")]
    pub field_id: String,
    pub text: String,
}
