// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowScheme {
    #[serde(rename = "defaultWorkflow", default, skip_serializing_if = "Option::is_none")]
    pub default_workflow: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "issueTypeMappings", default, skip_serializing_if = "Option::is_none")]
    pub issue_type_mappings: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "issueTypes", default, skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "lastModified", default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "lastModifiedUser", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "originalDefaultWorkflow", default, skip_serializing_if = "Option::is_none")]
    pub original_default_workflow: Option<String>,
    #[serde(rename = "originalIssueTypeMappings", default, skip_serializing_if = "Option::is_none")]
    pub original_issue_type_mappings: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "updateDraftIfNeeded", default, skip_serializing_if = "Option::is_none")]
    pub update_draft_if_needed: Option<bool>,
}
