// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details about an issue.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Issue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changelog: Option<PageOfChangelogs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editmeta: Option<IssueUpdateMetadata>,
    /// Expand options that include additional issue details in the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Box<IssueFields>>,
    #[serde(rename = "fieldsToInclude", default, skip_serializing_if = "Option::is_none")]
    pub fields_to_include: Option<IncludedFields>,
    /// The ID of the issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of the issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
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
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// The transitions that can be performed on the issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<IssueTransition>>,
    /// The versions of each field on the issue.
    #[serde(rename = "versionedRepresentations", default, skip_serializing_if = "Option::is_none")]
    pub versioned_representations: Option<std::collections::HashMap<String, serde_json::Value>>,
}
