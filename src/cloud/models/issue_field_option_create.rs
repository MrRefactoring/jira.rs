// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueFieldOptionCreate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<IssueFieldOptionConfiguration>,
    /// The properties of the option as arbitrary key-value pairs. These properties can be searched using JQL, if the extractions (see <https://developer.atlassian.com/cloud/jira/platform/modules/issue-field-option-property-index/>) are defined in the descriptor for the issue field module.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The option's name, which is displayed in Jira.
    pub value: String,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}
