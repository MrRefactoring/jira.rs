// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateCustomFieldRequest {
    /// The custom field ID.
    #[serde(rename = "customFieldId")]
    pub custom_field_id: i64,
    /// Allows filtering issues based on their values for the custom field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<bool>,
}
