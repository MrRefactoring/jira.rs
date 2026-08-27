// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DarkFeatures {
    #[serde(rename = "siteFeatures", default, skip_serializing_if = "Option::is_none")]
    pub site_features: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "systemFeatures", default, skip_serializing_if = "Option::is_none")]
    pub system_features: Option<std::collections::HashMap<String, serde_json::Value>>,
}
