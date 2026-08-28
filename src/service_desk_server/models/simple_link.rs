// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SimpleLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "iconClass", default, skip_serializing_if = "Option::is_none")]
    pub icon_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "styleClass", default, skip_serializing_if = "Option::is_none")]
    pub style_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}
