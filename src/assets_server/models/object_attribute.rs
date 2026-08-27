// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObjectAttribute {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "objectTypeAttribute", default, skip_serializing_if = "Option::is_none")]
    pub object_type_attribute: Option<ObjectTypeAttribute>,
    #[serde(rename = "objectTypeAttributeId", default, skip_serializing_if = "Option::is_none")]
    pub object_type_attribute_id: Option<i64>,
    #[serde(rename = "objectAttributeValues", default, skip_serializing_if = "Option::is_none")]
    pub object_attribute_values: Option<Vec<Box<ObjectAttributeValue>>>,
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<i64>,
}
