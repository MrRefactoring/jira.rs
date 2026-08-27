// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// An issue ID with entity property values. See [Entity properties](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/) for more information.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueEntityPropertiesForMultiUpdate {
    /// The ID of the issue.
    #[serde(rename = "issueID", default, skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<i64>,
    /// Entity properties to set on the issue. The maximum length of an issue property value is 32768 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
