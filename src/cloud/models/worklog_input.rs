// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A document in Atlassian Document Format, or a string of wiki markup — a string is sent to the v2 endpoint that parses it, and the result is read back as a document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum WorklogInputComment {
    Document(Document),
    Variant1(String),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// Details of a worklog.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WorklogInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<UserDetails>,
    /// A document in Atlassian Document Format, or a string of wiki markup — a string is sent to the v2 endpoint that parses it, and the result is read back as a document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<WorklogInputComment>,
    /// The datetime on which the worklog was created.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// The datetime on which the worklog was created.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub created: Option<String>,
    /// The ID of the worklog record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID of the issue this worklog is for.
    #[serde(rename = "issueId", default, skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
    /// Details of properties for the worklog. Optional when creating or updating a worklog.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<EntityProperty>>,
    /// The URL of the worklog item.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// The datetime on which the worklog effort was started. Required when creating a worklog. Optional when updating a worklog.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub started: Option<chrono::DateTime<chrono::Utc>>,
    /// The datetime on which the worklog effort was started. Required when creating a worklog. Optional when updating a worklog.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub started: Option<String>,
    /// The time spent working on the issue as days (#d), hours (#h), or minutes (#m or #). Required when creating a worklog if `timeSpentSeconds` isn't provided. Optional when updating a worklog. Cannot be provided if `timeSpentSecond` is provided.
    #[serde(rename = "timeSpent", default, skip_serializing_if = "Option::is_none")]
    pub time_spent: Option<String>,
    /// The time in seconds spent working on the issue. Required when creating a worklog if `timeSpent` isn't provided. Optional when updating a worklog. Cannot be provided if `timeSpent` is provided.
    #[serde(rename = "timeSpentSeconds", default, skip_serializing_if = "Option::is_none")]
    pub time_spent_seconds: Option<i64>,
    #[serde(rename = "updateAuthor", default, skip_serializing_if = "Option::is_none")]
    pub update_author: Option<UserDetails>,
    /// The datetime on which the worklog was last updated.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
    /// The datetime on which the worklog was last updated.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub updated: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for WorklogInput {
    const FIELDS: &'static [&'static str] = &[
        "author",
        "comment",
        "created",
        "id",
        "issueId",
        "properties",
        "self",
        "started",
        "timeSpent",
        "timeSpentSeconds",
        "updateAuthor",
        "updated",
        "visibility",
    ];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}
