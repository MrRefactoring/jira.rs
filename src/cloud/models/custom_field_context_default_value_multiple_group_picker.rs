// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueMultipleGroupPickerType {
    #[serde(rename = "grouppicker.multiple")]
    GrouppickerMultiple,
}

/// The default value for a multiple group picker custom field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldContextDefaultValueMultipleGroupPicker {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The IDs of the default groups.
    #[serde(rename = "groupIds")]
    pub group_ids: Vec<String>,
    pub r#type: CustomFieldContextDefaultValueMultipleGroupPickerType,
}
