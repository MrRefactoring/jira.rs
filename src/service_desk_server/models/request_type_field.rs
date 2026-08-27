// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestTypeField {
    #[serde(rename = "fieldId", default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "validValues", default, skip_serializing_if = "Option::is_none")]
    pub valid_values: Option<Vec<RequestTypeFieldValue>>,
    #[serde(rename = "jiraSchema", default, skip_serializing_if = "Option::is_none")]
    pub jira_schema: Option<JsonType>,
}
