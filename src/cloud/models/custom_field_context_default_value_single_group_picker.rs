// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueSingleGroupPickerType {
    #[serde(rename = "grouppicker.single")]
    GrouppickerSingle,
}

/// The default value for a group picker custom field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueSingleGroupPicker {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The ID of the the default group.
    #[serde(rename = "groupId")]
    pub group_id: String,
    pub r#type: CustomFieldContextDefaultValueSingleGroupPickerType,
}
