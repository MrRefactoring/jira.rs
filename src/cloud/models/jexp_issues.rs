// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The JQL specifying the issues available in the evaluated Jira expression under the `issues` context variable.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JexpIssues {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jql: Option<JexpJqlIssues>,
}
