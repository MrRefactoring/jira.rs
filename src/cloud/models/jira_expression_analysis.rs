// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details about the analysed Jira expression.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JiraExpressionAnalysis {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complexity: Option<JiraExpressionComplexity>,
    /// A list of validation errors. Not included if the expression is valid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<JiraExpressionValidationError>>,
    /// The analysed expression.
    pub expression: String,
    /// EXPERIMENTAL. The inferred type of the expression.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Whether the expression is valid and the interpreter will evaluate it. Note that the expression may fail at runtime (for example, if it executes too many expensive operations).
    pub valid: bool,
}
