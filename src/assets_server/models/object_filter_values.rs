// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObjectFilterValues {
    #[serde(rename = "objectTypeAttributeId", default, skip_serializing_if = "Option::is_none")]
    pub object_type_attribute_id: Option<i64>,
    #[serde(rename = "selectedValues", default, skip_serializing_if = "Option::is_none")]
    pub selected_values: Option<Vec<String>>,
    #[serde(rename = "filterByObjectType", default, skip_serializing_if = "Option::is_none")]
    pub filter_by_object_type: Option<bool>,
}
