// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A page of changelogs.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueChangelog {
    /// The list of changelogs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub histories: Option<Vec<Changelog>>,
    /// The maximum number of results that could be on the page.
    #[serde(rename = "maxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// The index of the first item returned on the page.
    #[serde(rename = "startAt", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// The number of results on the page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

/// A list of editable field details.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueEditmeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// Details about an issue.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Issue {
    /// A page of changelogs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changelog: Option<IssueChangelog>,
    /// A list of editable field details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editmeta: Option<IssueEditmeta>,
    /// Expand options that include additional issue details in the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "fieldsToInclude", default, skip_serializing_if = "Option::is_none")]
    pub fields_to_include: Option<IncludedFields>,
    /// The ID of the issue.
    pub id: String,
    /// The key of the issue.
    pub key: String,
    /// The ID and name of each field present on the issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations: Option<Operations>,
    /// Details of the issue properties identified in the request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The rendered value of each field present on the issue.
    #[serde(rename = "renderedFields", default, skip_serializing_if = "Option::is_none")]
    pub rendered_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The schema describing each field present on the issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The URL of the issue details.
    #[serde(rename = "self")]
    pub self_: String,
    /// The transitions that can be performed on the issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<IssueTransition>>,
    /// The versions of each field on the issue.
    #[serde(rename = "versionedRepresentations", default, skip_serializing_if = "Option::is_none")]
    pub versioned_representations: Option<std::collections::HashMap<String, serde_json::Value>>,
}
