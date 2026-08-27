// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The error type.
    pub enum JiraExpressionValidationErrorType {
        Syntax => "syntax",
        Type => "type",
        Other2 => "other",
    }
}

/// Details about syntax and type errors. The error details apply to the entire expression, unless the object includes:
///
///  *  `line` and `column`
///  *  `expression`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraExpressionValidationError {
    /// The text column in which the error occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    /// The part of the expression in which the error occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// The text line in which the error occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    /// Details about the error.
    pub message: String,
    /// The error type.
    pub r#type: JiraExpressionValidationErrorType,
}
