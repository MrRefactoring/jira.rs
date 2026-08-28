// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectJson {
    #[serde(rename = "avatarUrls", default, skip_serializing_if = "Option::is_none")]
    pub avatar_urls: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "projectCategory", default, skip_serializing_if = "Option::is_none")]
    pub project_category: Option<ProjectCategoryJson>,
    #[serde(rename = "projectTypeKey", default, skip_serializing_if = "Option::is_none")]
    pub project_type_key: Option<String>,
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
