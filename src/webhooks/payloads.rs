//! The bodies Jira posts to a webhook.
//!
//! How much of this Atlassian documents, plainly: one complete payload, the one for issue events, and of the rest
//! only that a callback carries "information about the entity associated with the event". Every entity below is
//! therefore optional — there was nothing to verify it against, and an optional field costs a guard rather than a
//! crash.

use serde::{Deserialize, Serialize};

use super::events::WebhookEvent;
use crate::agile::{Board, Sprint};
use crate::cloud::{
    Attachment, Changelog, Comment, Filter, Issue, IssueLink, IssueTypeDetails, Project, UserDetails, Version, Worklog,
};

/// One delivery, whatever the event.
///
/// A single shape rather than an enum over the events. An enum would have to reject a body whose `webhookEvent` it
/// does not know, and for a webhook receiver that is an outage: switch on [`webhook_event`](Self::webhook_event) and
/// read the entity the event concerns.
///
/// Unknown fields are ignored, so a site's custom configuration and an app's additions do not stop a body parsing.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookPayload {
    /// When Jira raised the event, in milliseconds since the epoch.
    pub timestamp: Option<i64>,
    /// The event, and the field to switch on.
    pub webhook_event: Option<WebhookEvent>,
    /// The registrations this delivery answered. Present only on a webhook registered through the REST API,
    /// where one event can match several at once.
    pub matched_webhook_ids: Option<Vec<i64>>,
    #[serde(rename = "issue_event_type_name")]
    /// Jira's own name for what happened, finer than the event: an edit, a comment and a transition all arrive
    /// as `jira:issue_updated` and are told apart only here. A string rather than an enum, because an administrator
    /// can add issue events to a site.
    pub issue_event_type_name: Option<String>,
    /// The issue as it stands after the change. On a deletion, as it stood before.
    pub issue: Option<Issue>,
    /// Who caused it. A scheduled change has no actor to name.
    pub user: Option<UserDetails>,
    /// What changed. Present on an update, absent on a creation that changed nothing.
    pub changelog: Option<Changelog>,
    /// The comment, when the update was someone commenting.
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
