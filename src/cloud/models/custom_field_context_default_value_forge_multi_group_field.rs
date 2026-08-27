// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueForgeMultiGroupFieldType {
    #[serde(rename = "forge.group.list")]
    ForgeGroupList,
}

/// The default value for a Forge collection of groups custom field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueForgeMultiGroupField {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The IDs of the default groups.
    #[serde(rename = "groupIds")]
    pub group_ids: Vec<String>,
    pub r#type: CustomFieldContextDefaultValueForgeMultiGroupFieldType,
}
