// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The JQL specifying the issues available in the evaluated Jira expression under the `issues` context variable. This bean will be replacing `JexpIssues` bean as part of new `evaluate` endpoint
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JexpEvaluateCtxIssues {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jql: Option<JexpEvaluateCtxJqlIssues>,
}
