// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Determines how to validate the JQL query and treat the validation results.
    pub enum JexpJqlIssuesValidation {
        Strict => "strict",
        Warn => "warn",
        None => "none",
    }
}

/// The JQL specifying the issues available in the evaluated Jira expression under the `issues` context variable. Not all issues returned by the JQL query are loaded, only those described by the `startAt` and `maxResults` properties. To determine whether it is necessary to iterate to ensure all the issues returned by the JQL query are evaluated, inspect `meta.issues.jql.count` in the response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JexpJqlIssues {
    /// The maximum number of issues to return from the JQL query. Inspect `meta.issues.jql.maxResults` in the response to ensure the maximum value has not been exceeded.
    #[serde(rename = "maxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// The JQL query.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// The index of the first issue to return from the JQL query.
    #[serde(rename = "startAt", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// Determines how to validate the JQL query and treat the validation results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<JexpJqlIssuesValidation>,
}
