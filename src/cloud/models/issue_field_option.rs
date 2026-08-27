// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of the options for a select list issue field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueFieldOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<IssueFieldOptionConfiguration>,
    /// The unique identifier for the option. This is only unique within the select field's set of options.
    pub id: i64,
    /// The properties of the object, as arbitrary key-value pairs. These properties can be searched using JQL, if the extractions (see [Issue Field Option Property Index](https://developer.atlassian.com/cloud/jira/platform/modules/issue-field-option-property-index/)) are defined in the descriptor for the issue field module.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The option's name, which is displayed in Jira.
    pub value: String,
}
