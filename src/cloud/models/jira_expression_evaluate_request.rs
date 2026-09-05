// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The request to evaluate a Jira expression. This bean will be replacing `JiraExpressionEvaluateRequest` as part of new `evaluate` endpoint
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraExpressionEvaluateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<JiraExpressionEvaluateContext>,
    /// The Jira expression to evaluate.
    pub expression: String,
}
