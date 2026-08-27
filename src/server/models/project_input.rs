// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum ProjectInputAssigneeType {
        ProjectLead => "PROJECT_LEAD",
        Unassigned => "UNASSIGNED",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectInput {
    #[serde(rename = "assigneeType", default, skip_serializing_if = "Option::is_none")]
    pub assignee_type: Option<ProjectInputAssigneeType>,
    #[serde(rename = "avatarId", default, skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    #[serde(rename = "categoryId", default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "issueSecurityScheme", default, skip_serializing_if = "Option::is_none")]
    pub issue_security_scheme: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lead: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "notificationScheme", default, skip_serializing_if = "Option::is_none")]
    pub notification_scheme: Option<i64>,
    #[serde(rename = "permissionScheme", default, skip_serializing_if = "Option::is_none")]
    pub permission_scheme: Option<i64>,
    #[serde(rename = "projectTemplateKey", default, skip_serializing_if = "Option::is_none")]
    pub project_template_key: Option<String>,
    #[serde(rename = "projectTypeKey", default, skip_serializing_if = "Option::is_none")]
    pub project_type_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "workflowSchemeId", default, skip_serializing_if = "Option::is_none")]
    pub workflow_scheme_id: Option<i64>,
}
