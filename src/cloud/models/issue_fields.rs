// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A document in Atlassian Document Format, or a string of wiki markup — a string is sent to the v2 endpoint that parses it, and Jira stores the document it made of it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum IssueFieldsDescription {
    Document(Document),
    Variant1(String),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// A document in Atlassian Document Format, or a string of wiki markup — a string is sent to the v2 endpoint that parses it, and Jira stores the document it made of it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum IssueFieldsEnvironment {
    Document(Document),
    Variant1(String),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The fields of an issue: the system fields by name, and every custom field by its `customfield_` key alongside them. Reading one, the fields the request did not ask for are absent; writing one, the fields left out are left as they were.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueFields {
    /// The one-line title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A document in Atlassian Document Format, or a string of wiki markup — a string is sent to the v2 endpoint that parses it, and Jira stores the document it made of it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<IssueFieldsDescription>,
    /// A document in Atlassian Document Format, or a string of wiki markup — a string is sent to the v2 endpoint that parses it, and Jira stores the document it made of it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<IssueFieldsEnvironment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuetype: Option<IssueTypeDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Resolution>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<UserDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reporter: Option<UserDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<UserDetails>,
    /// When the issue was created.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// When the issue was created.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub created: Option<String>,
    /// When the issue last changed.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
    /// When the issue last changed.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub updated: Option<String>,
    /// When the issue was resolved.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub resolutiondate: Option<chrono::DateTime<chrono::Utc>>,
    /// When the issue was resolved.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub resolutiondate: Option<String>,
    /// When the issue last moved between status categories.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub statuscategorychangedate: Option<chrono::DateTime<chrono::Utc>>,
    /// When the issue last moved between status categories.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub statuscategorychangedate: Option<String>,
    /// When the current user last opened the issue.
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "lastViewed",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub last_viewed: Option<chrono::DateTime<chrono::Utc>>,
    /// When the current user last opened the issue.
    #[cfg(not(feature = "chrono"))]
    #[serde(
        rename = "lastViewed",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub last_viewed: Option<String>,
    /// The due date, as `YYYY-MM-DD`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duedate: Option<String>,
    /// The labels on the issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<ProjectComponent>>,
    #[serde(rename = "fixVersions", default, skip_serializing_if = "Option::is_none")]
    pub fix_versions: Option<Vec<Version>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<Version>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<Issue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtasks: Option<Vec<Issue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuelinks: Option<Vec<IssueLink>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<Attachment>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<PageOfComments>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worklog: Option<PageOfWorklogs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timetracking: Option<TimeTrackingDetails>,
    /// Seconds logged on the issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timespent: Option<i64>,
    /// Seconds still estimated on the issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeestimate: Option<i64>,
    /// Seconds the issue was first estimated at.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeoriginalestimate: Option<i64>,
    /// Seconds logged on the issue and its subtasks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregatetimespent: Option<i64>,
    /// Seconds still estimated on the issue and its subtasks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregatetimeestimate: Option<i64>,
    /// Seconds the issue and its subtasks were first estimated at.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregatetimeoriginalestimate: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<IssueProgress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregateprogress: Option<IssueProgress>,
    /// Time spent as a percentage of the original estimate, or -1 where nothing was estimated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workratio: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub votes: Option<Votes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watches: Option<Watchers>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security: Option<SecurityLevel>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for IssueFields {
    const FIELDS: &'static [&'static str] = &[
        "summary",
        "description",
        "environment",
        "issuetype",
        "project",
        "status",
        "priority",
        "resolution",
        "assignee",
        "reporter",
        "creator",
        "created",
        "updated",
        "resolutiondate",
        "statuscategorychangedate",
        "lastViewed",
        "duedate",
        "labels",
        "components",
        "fixVersions",
        "versions",
        "parent",
        "subtasks",
        "issuelinks",
        "attachment",
        "comment",
        "worklog",
        "timetracking",
        "timespent",
        "timeestimate",
        "timeoriginalestimate",
        "aggregatetimespent",
        "aggregatetimeestimate",
        "aggregatetimeoriginalestimate",
        "progress",
        "aggregateprogress",
        "workratio",
        "votes",
        "watches",
        "security",
    ];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}
