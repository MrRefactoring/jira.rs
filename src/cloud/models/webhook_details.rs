// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum WebhookDetailsEvents {
        JiraIssueCreated => "jira:issue_created",
        JiraIssueUpdated => "jira:issue_updated",
        JiraIssueDeleted => "jira:issue_deleted",
        CommentCreated => "comment_created",
        CommentUpdated => "comment_updated",
        CommentDeleted => "comment_deleted",
        IssuePropertySet => "issue_property_set",
        IssuePropertyDeleted => "issue_property_deleted",
        SprintCreated => "sprint_created",
        SprintUpdated => "sprint_updated",
        SprintClosed => "sprint_closed",
        SprintDeleted => "sprint_deleted",
        SprintStarted => "sprint_started",
        JiraVersionReleased => "jira:version_released",
        JiraVersionUnreleased => "jira:version_unreleased",
        JiraVersionCreated => "jira:version_created",
        JiraVersionMoved => "jira:version_moved",
        JiraVersionUpdated => "jira:version_updated",
        JiraVersionMerged => "jira:version_merged",
        JiraVersionDeleted => "jira:version_deleted",
    }
}

/// A list of webhooks.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WebhookDetails {
    /// The Jira events that trigger the webhook.
    pub events: Vec<WebhookDetailsEvents>,
    /// A list of field IDs. When the issue changelog contains any of the fields, the webhook `jira:issue_updated` is sent. If this parameter is not present, the app is notified about all field updates.
    #[serde(rename = "fieldIdsFilter", default, skip_serializing_if = "Option::is_none")]
    pub field_ids_filter: Option<Vec<String>>,
    /// A list of issue property keys. A change of those issue properties triggers the `issue_property_set` or `issue_property_deleted` webhooks. If this parameter is not present, the app is notified about all issue property updates.
    #[serde(rename = "issuePropertyKeysFilter", default, skip_serializing_if = "Option::is_none")]
    pub issue_property_keys_filter: Option<Vec<String>>,
    /// The JQL filter that specifies which issues the webhook is sent for. Only a subset of JQL can be used. The supported elements are:
    ///
    ///  *  Fields: `issueKey`, `project`, `issuetype`, `status`, `assignee`, `reporter`, `issue.property`, and `cf[id]`. For custom fields (`cf[id]`), only the epic label custom field is supported.".
    ///  *  Operators: `=`, `!=`, `IN`, and `NOT IN`.
    #[serde(rename = "jqlFilter")]
    pub jql_filter: String,
}
