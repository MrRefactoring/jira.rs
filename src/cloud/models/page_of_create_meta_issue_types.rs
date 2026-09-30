// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A page of CreateMetaIssueTypes.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PageOfCreateMetaIssueTypes {
    #[serde(rename = "createMetaIssueType", default, skip_serializing_if = "Option::is_none")]
    pub create_meta_issue_type: Option<Vec<IssueTypeIssueCreateMetadata>>,
    /// The list of CreateMetaIssueType.
    #[serde(rename = "issueTypes", default, skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Vec<IssueTypeIssueCreateMetadata>>,
    /// The maximum number of items to return per page.
    #[serde(rename = "maxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// The index of the first item returned.
    #[serde(rename = "startAt", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// The total number of items in all pages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for PageOfCreateMetaIssueTypes {
    const FIELDS: &'static [&'static str] = &["createMetaIssueType", "issueTypes", "maxResults", "startAt", "total"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}
