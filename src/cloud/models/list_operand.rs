// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// An operand that is a list of values.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListOperand {
    /// Encoded operand, which can be used directly in a JQL query.
    #[serde(rename = "encodedOperand", default, skip_serializing_if = "Option::is_none")]
    pub encoded_operand: Option<String>,
    /// The list of operand values.
    pub values: Vec<JqlQueryUnitaryOperand>,
}
