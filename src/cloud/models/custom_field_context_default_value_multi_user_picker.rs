// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueMultiUserPickerType {
    #[serde(rename = "multi.user.select")]
    MultiUserSelect,
}

/// The default value for a User Picker (multiple) custom field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldContextDefaultValueMultiUserPicker {
    /// The IDs of the default users.
    #[serde(rename = "accountIds")]
    pub account_ids: Vec<String>,
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    pub r#type: CustomFieldContextDefaultValueMultiUserPickerType,
}
