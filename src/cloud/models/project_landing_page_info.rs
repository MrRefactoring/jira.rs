// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectLandingPageInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "boardId", default, skip_serializing_if = "Option::is_none")]
    pub board_id: Option<i64>,
    #[serde(rename = "boardName", default, skip_serializing_if = "Option::is_none")]
    pub board_name: Option<String>,
    #[serde(rename = "projectKey", default, skip_serializing_if = "Option::is_none")]
    pub project_key: Option<String>,
    #[serde(rename = "projectType", default, skip_serializing_if = "Option::is_none")]
    pub project_type: Option<String>,
    #[serde(rename = "queueCategory", default, skip_serializing_if = "Option::is_none")]
    pub queue_category: Option<String>,
    #[serde(rename = "queueId", default, skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i64>,
    #[serde(rename = "queueName", default, skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
    #[serde(rename = "simpleBoard", default, skip_serializing_if = "Option::is_none")]
    pub simple_board: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simplified: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
