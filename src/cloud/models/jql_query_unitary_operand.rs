// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// An operand that can be part of a list operand.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum JqlQueryUnitaryOperand {
    ValueOperand(ValueOperand),
    FunctionOperand(FunctionOperand),
    KeywordOperand(KeywordOperand),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}
