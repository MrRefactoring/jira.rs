// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The default assignee when creating issues for this project.
    pub enum UpdateProjectDetailsAssigneeType {
        ProjectLead => "PROJECT_LEAD",
        Unassigned => "UNASSIGNED",
    }
}

/// Details about the project.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateProjectDetails {
    /// The default assignee when creating issues for this project.
    #[serde(rename = "assigneeType", default, skip_serializing_if = "Option::is_none")]
    pub assignee_type: Option<UpdateProjectDetailsAssigneeType>,
    /// An integer value for the project's avatar.
    #[serde(rename = "avatarId", default, skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    /// The ID of the project's category. A complete list of category IDs is found using the [Get all project categories](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-project-category/#api-rest-api-3-projectCategory-get) operation. To remove the project category from the project, set the value to `-1.`
    #[serde(rename = "categoryId", default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    /// A brief description of the project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the issue security scheme for the project, which enables you to control who can and cannot view issues. Use the [Get issue security schemes](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issue-security-schemes/#api-rest-api-3-issuesecurityschemes-get) resource to get all issue security scheme IDs.
    #[serde(rename = "issueSecurityScheme", default, skip_serializing_if = "Option::is_none")]
    pub issue_security_scheme: Option<i64>,
    /// Project keys must be unique and start with an uppercase letter followed by one or more uppercase alphanumeric characters. The maximum length is 10 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The account ID of the project lead. Cannot be provided with `lead`.
    #[serde(rename = "leadAccountId", default, skip_serializing_if = "Option::is_none")]
    pub lead_account_id: Option<String>,
    /// The name of the project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The ID of the notification scheme for the project. Use the [Get notification schemes](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-notification-schemes/#api-rest-api-3-notificationscheme-get) resource to get a list of notification scheme IDs.
    #[serde(rename = "notificationScheme", default, skip_serializing_if = "Option::is_none")]
    pub notification_scheme: Option<i64>,
    /// The ID of the permission scheme for the project. Use the [Get all permission schemes](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-permission-schemes/#api-rest-api-3-permissionscheme-get) resource to see a list of all permission scheme IDs.
    #[serde(rename = "permissionScheme", default, skip_serializing_if = "Option::is_none")]
    pub permission_scheme: Option<i64>,
    /// Previous project keys to be released from the current project. Released keys must belong to the current project and not contain the current project key
    #[serde(rename = "releasedProjectKeys", default, skip_serializing_if = "Option::is_none")]
    pub released_project_keys: Option<Vec<String>>,
    /// A link to information about this project, such as project documentation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
