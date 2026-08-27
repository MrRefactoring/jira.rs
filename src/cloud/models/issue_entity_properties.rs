// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Lists of issues and entity properties. See [Entity properties](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/) for more information.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueEntityProperties {
    /// A list of entity property IDs.
    #[serde(rename = "entitiesIds", default, skip_serializing_if = "Option::is_none")]
    pub entities_ids: Option<Vec<i64>>,
    /// A list of entity property keys and values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
