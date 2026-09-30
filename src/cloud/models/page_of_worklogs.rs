// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Paginated list of worklog details
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PageOfWorklogs {
    /// The maximum number of results that could be on the page.
    #[serde(rename = "maxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// The index of the first item returned on the page.
    #[serde(rename = "startAt", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// The number of results on the page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// List of worklogs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worklogs: Option<Vec<Worklog>>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for PageOfWorklogs {
    const FIELDS: &'static [&'static str] = &["maxResults", "startAt", "total", "worklogs"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}
