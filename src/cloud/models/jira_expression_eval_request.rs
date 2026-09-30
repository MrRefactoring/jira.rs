// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JiraExpressionEvalRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<JiraExpressionEvalContext>,
    /// The Jira expression to evaluate.
    pub expression: String,
}
