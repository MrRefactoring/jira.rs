// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Field {
    #[serde(rename = "clauseNames", default, skip_serializing_if = "Option::is_none")]
    pub clause_names: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub navigable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orderable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<JsonType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub searchable: Option<bool>,
}
