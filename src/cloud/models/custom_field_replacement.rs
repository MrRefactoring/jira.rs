// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details about the replacement for a deleted version.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomFieldReplacement {
    /// The ID of the custom field in which to replace the version number.
    #[serde(rename = "customFieldId", default, skip_serializing_if = "Option::is_none")]
    pub custom_field_id: Option<i64>,
    /// The version number to use as a replacement for the deleted version.
    #[serde(rename = "moveTo", default, skip_serializing_if = "Option::is_none")]
    pub move_to: Option<i64>,
}
