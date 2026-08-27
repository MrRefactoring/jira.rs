// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestTypeIconLink {
    #[serde(rename = "iconUrls", default, skip_serializing_if = "Option::is_none")]
    pub icon_urls: Option<std::collections::HashMap<String, serde_json::Value>>,
}
