use serde::{Deserialize, Serialize};

use super::events::WebhookEvent;
use crate::agile::{Board, Sprint};
use crate::cloud::{
    Attachment, Changelog, Comment, Filter, Issue, IssueLink, IssueTypeDetails, Project, UserDetails, Version, Worklog,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookPayload {
    pub timestamp: Option<i64>,
    pub webhook_event: Option<WebhookEvent>,
    pub matched_webhook_ids: Option<Vec<i64>>,
    #[serde(rename = "issue_event_type_name")]
    pub issue_event_type_name: Option<String>,
    pub issue: Option<Issue>,
    pub user: Option<UserDetails>,
    pub changelog: Option<Changelog>,
    pub comment: Option<Comment>,
    pub worklog: Option<Worklog>,
    pub attachment: Option<Attachment>,
    pub issue_link: Option<IssueLink>,
    pub issue_type: Option<IssueTypeDetails>,
    pub project: Option<Project>,
    pub version: Option<Version>,
    pub filter: Option<Filter>,
    pub sprint: Option<Sprint>,
    pub board: Option<Board>,
}
