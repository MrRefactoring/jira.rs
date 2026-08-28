// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A status category.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StatusCategory {
    /// The name of the color used to represent the status category.
    #[serde(rename = "colorName", default, skip_serializing_if = "Option::is_none")]
    pub color_name: Option<String>,
    /// The ID of the status category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The key of the status category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of the status category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the status category.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}
