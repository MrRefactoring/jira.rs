// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The result of evaluating a Jira expression.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JiraExpressionResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<JiraExpressionEvaluationMetaData>,
    /// The value of the evaluated expression. It may be a primitive JSON value or a Jira REST API object. (Some expressions do not produce any meaningful results—for example, an expression that returns a lambda function—if that's the case a simple string representation is returned. These string representations should not be relied upon and may change without notice.)
    pub value: serde_json::Value,
}
