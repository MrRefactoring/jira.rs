// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomFieldReplacement {
    #[serde(rename = "customFieldId", default, skip_serializing_if = "Option::is_none")]
    pub custom_field_id: Option<i64>,
    #[serde(rename = "moveTo", default, skip_serializing_if = "Option::is_none")]
    pub move_to: Option<i64>,
}
