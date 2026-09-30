// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum WebhookEvents {
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

/// A webhook.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Webhook {
    /// The Jira events that trigger the webhook.
    pub events: Vec<WebhookEvents>,
    /// The date after which the webhook is no longer sent. Use [Extend webhook life](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-webhooks/#api-rest-api-3-webhook-refresh-put) to extend the date.
    #[serde(rename = "expirationDate", default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<i64>,
    /// A list of field IDs. When the issue changelog contains any of the fields, the webhook `jira:issue_updated` is sent. If this parameter is not present, the app is notified about all field updates.
    #[serde(rename = "fieldIdsFilter", default, skip_serializing_if = "Option::is_none")]
    pub field_ids_filter: Option<Vec<String>>,
    /// The ID of the webhook.
    pub id: i64,
    /// A list of issue property keys. A change of those issue properties triggers the `issue_property_set` or `issue_property_deleted` webhooks. If this parameter is not present, the app is notified about all issue property updates.
    #[serde(rename = "issuePropertyKeysFilter", default, skip_serializing_if = "Option::is_none")]
    pub issue_property_keys_filter: Option<Vec<String>>,
    /// The JQL filter that specifies which issues the webhook is sent for.
    #[serde(rename = "jqlFilter")]
    pub jql_filter: String,
    /// The URL that specifies where the webhooks are sent.
    pub url: String,
}
