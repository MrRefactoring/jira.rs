// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriorityScheme {
    #[serde(rename = "defaultOptionId", default, skip_serializing_if = "Option::is_none")]
    pub default_option_id: Option<String>,
    #[serde(rename = "defaultScheme", default, skip_serializing_if = "Option::is_none")]
    pub default_scheme: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "optionIds", default, skip_serializing_if = "Option::is_none")]
    pub option_ids: Option<Vec<String>>,
    #[serde(rename = "projectKeys", default, skip_serializing_if = "Option::is_none")]
    pub project_keys: Option<Vec<String>>,
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
