// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FilterRegistration {
    #[serde(rename = "className", default, skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    #[serde(rename = "initParameters", default, skip_serializing_if = "Option::is_none")]
    pub init_parameters: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "servletNameMappings", default, skip_serializing_if = "Option::is_none")]
    pub servlet_name_mappings: Option<Vec<String>>,
    #[serde(rename = "urlPatternMappings", default, skip_serializing_if = "Option::is_none")]
    pub url_pattern_mappings: Option<Vec<String>>,
}
