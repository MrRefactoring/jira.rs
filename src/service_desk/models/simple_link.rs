// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details about the operations available in this version.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SimpleLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "iconClass", default, skip_serializing_if = "Option::is_none")]
    pub icon_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "styleClass", default, skip_serializing_if = "Option::is_none")]
    pub style_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}
