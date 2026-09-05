// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of a filter for exporting archived issues.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ArchivedIssuesFilterRequest {
    /// List archived issues archived by a specified account ID.
    #[serde(rename = "archivedBy", default, skip_serializing_if = "Option::is_none")]
    pub archived_by: Option<Vec<String>>,
    #[serde(rename = "archivedDateRange", default, skip_serializing_if = "Option::is_none")]
    pub archived_date_range: Option<DateRangeFilterRequest>,
    /// List archived issues with a specified issue type ID.
    #[serde(rename = "issueTypes", default, skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Vec<String>>,
    /// List archived issues with a specified project key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<String>>,
    /// List archived issues where the reporter is a specified account ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reporters: Option<Vec<String>>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for ArchivedIssuesFilterRequest {
    const FIELDS: &'static [&'static str] = &["archivedBy", "archivedDateRange", "issueTypes", "projects", "reporters"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}
