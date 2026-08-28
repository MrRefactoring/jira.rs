// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueSingleVersionPickerType {
    #[serde(rename = "version.single")]
    VersionSingle,
}

/// The default value for a version picker custom field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldContextDefaultValueSingleVersionPicker {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    pub r#type: CustomFieldContextDefaultValueSingleVersionPickerType,
    /// The ID of the default version.
    #[serde(rename = "versionId")]
    pub version_id: String,
    /// The order the pickable versions are displayed in. If not provided, the released-first order is used. Available version orders are `"releasedFirst"` and `"unreleasedFirst"`.
    #[serde(rename = "versionOrder", default, skip_serializing_if = "Option::is_none")]
    pub version_order: Option<String>,
}
