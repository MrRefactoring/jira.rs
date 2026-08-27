// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutoCompleteResponseVisibleFieldNames {
    pub value: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub operators: Vec<String>,
    pub types: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub searchable: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orderable: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cfid: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutoCompleteResponseVisibleFunctionNames {
    pub value: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub types: Vec<String>,
    #[serde(rename = "isList", default, skip_serializing_if = "Option::is_none")]
    pub is_list: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutoCompleteResponse {
    #[serde(rename = "jqlReservedWords", default, skip_serializing_if = "Option::is_none")]
    pub jql_reserved_words: Option<Vec<String>>,
    #[serde(rename = "visibleFieldNames", default, skip_serializing_if = "Option::is_none")]
    pub visible_field_names: Option<Vec<AutoCompleteResponseVisibleFieldNames>>,
    #[serde(rename = "visibleFunctionNames", default, skip_serializing_if = "Option::is_none")]
    pub visible_function_names: Option<Vec<AutoCompleteResponseVisibleFunctionNames>>,
}
