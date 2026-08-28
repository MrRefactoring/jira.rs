// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueForgeGroupFieldType {
    #[serde(rename = "forge.group")]
    ForgeGroup,
}

/// The default value for a Forge group custom field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldContextDefaultValueForgeGroupField {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The ID of the the default group.
    #[serde(rename = "groupId")]
    pub group_id: String,
    pub r#type: CustomFieldContextDefaultValueForgeGroupFieldType,
}
