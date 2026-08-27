// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HttpSession {
    #[serde(rename = "attributeNames", default, skip_serializing_if = "Option::is_none")]
    pub attribute_names: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "creationTime", default, skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastAccessedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_accessed_time: Option<i64>,
    #[serde(rename = "maxInactiveInterval", default, skip_serializing_if = "Option::is_none")]
    pub max_inactive_interval: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new: Option<bool>,
    #[serde(rename = "servletContext", default, skip_serializing_if = "Option::is_none")]
    pub servlet_context: Option<ServletContext>,
}
