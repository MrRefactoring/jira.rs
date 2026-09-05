// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A document in Atlassian Document Format, or a string of wiki markup — a string is sent to the v2 endpoint that parses it, and the result is read back as a document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CommentInputBody {
    Document(Document),
    Variant1(String),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// A comment.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CommentInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<UserDetails>,
    /// A document in Atlassian Document Format, or a string of wiki markup — a string is sent to the v2 endpoint that parses it, and the result is read back as a document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<CommentInputBody>,
    /// The date and time at which the comment was created.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time at which the comment was created.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub created: Option<String>,
    /// The ID of the comment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the comment was added from an email sent by a person who is not part of the issue. See [Allow external emails to be added as comments on issues](https://support.atlassian.com/jira-service-management-cloud/docs/allow-external-emails-to-be-added-as-comments-on-issues/)for information on setting up this feature.
    #[serde(rename = "jsdAuthorCanSeeRequest", default, skip_serializing_if = "Option::is_none")]
    pub jsd_author_can_see_request: Option<bool>,
    /// Whether the comment is visible in Jira Service Desk. Defaults to true when comments are created in the Jira Cloud Platform. This includes when the site doesn't use Jira Service Desk or the project isn't a Jira Service Desk project and, therefore, there is no Jira Service Desk for the issue to be visible on. To create a comment with its visibility in Jira Service Desk set to false, use the Jira Service Desk REST API [Create request comment](https://developer.atlassian.com/cloud/jira/service-desk/rest/#api-rest-servicedeskapi-request-issueIdOrKey-comment-post) operation.
    #[serde(rename = "jsdPublic", default, skip_serializing_if = "Option::is_none")]
    pub jsd_public: Option<bool>,
    /// A list of comment properties. Optional on create and update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<EntityProperty>>,
    /// The rendered version of the comment.
    #[serde(rename = "renderedBody", default, skip_serializing_if = "Option::is_none")]
    pub rendered_body: Option<String>,
    /// The URL of the comment.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "updateAuthor", default, skip_serializing_if = "Option::is_none")]
    pub update_author: Option<UserDetails>,
    /// The date and time at which the comment was updated last.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time at which the comment was updated last.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub updated: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for CommentInput {
    const FIELDS: &'static [&'static str] = &[
        "author",
        "body",
        "created",
        "id",
        "jsdAuthorCanSeeRequest",
        "jsdPublic",
        "properties",
        "renderedBody",
        "self",
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
