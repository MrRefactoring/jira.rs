// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A container for the watch status of a list of issues.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkIssueIsWatching {
    /// The map of issue ID to boolean watch status.
    #[serde(rename = "issuesIsWatching", default, skip_serializing_if = "Option::is_none")]
    pub issues_is_watching: Option<std::collections::HashMap<String, serde_json::Value>>,
}
