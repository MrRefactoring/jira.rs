// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// An operand that is a user-provided value.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ValueOperand {
    /// Encoded value, which can be used directly in a JQL query.
    #[serde(rename = "encodedValue", default, skip_serializing_if = "Option::is_none")]
    pub encoded_value: Option<String>,
    /// The operand value.
    pub value: String,
}
