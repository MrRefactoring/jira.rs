// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PrioritySchemeUpdate {
    #[serde(rename = "defaultOptionId", default, skip_serializing_if = "Option::is_none")]
    pub default_option_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "optionIds", default, skip_serializing_if = "Option::is_none")]
    pub option_ids: Option<Vec<String>>,
}
