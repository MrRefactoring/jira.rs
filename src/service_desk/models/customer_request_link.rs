// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomerRequestLink {
    /// Jira agent view URL for the request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// REST API URL for the request.
    #[serde(rename = "jiraRest", default, skip_serializing_if = "Option::is_none")]
    pub jira_rest: Option<String>,
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// Web URL for the request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
}
