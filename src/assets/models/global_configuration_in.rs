// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalConfigurationIn {
    #[serde(rename = "allowOtherObjectSchema", default, skip_serializing_if = "Option::is_none")]
    pub allow_other_object_schema: Option<bool>,
    #[serde(rename = "validateQuickCreate", default, skip_serializing_if = "Option::is_none")]
    pub validate_quick_create: Option<bool>,
    #[serde(rename = "quickCreateObjects", default, skip_serializing_if = "Option::is_none")]
    pub quick_create_objects: Option<bool>,
}
