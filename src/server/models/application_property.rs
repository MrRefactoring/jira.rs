// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApplicationProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[serde(rename = "allowedValues", default, skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
}
