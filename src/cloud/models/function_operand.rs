// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// An operand that is a function. See [Advanced searching - functions reference](https://confluence.atlassian.com/x/dwiiLQ) for more information about JQL functions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FunctionOperand {
    /// The list of function arguments.
    pub arguments: Vec<String>,
    /// Encoded operand, which can be used directly in a JQL query.
    #[serde(rename = "encodedOperand", default, skip_serializing_if = "Option::is_none")]
    pub encoded_operand: Option<String>,
    /// The name of the function.
    pub function: String,
}
