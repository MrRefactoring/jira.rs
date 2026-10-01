// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::Serialize;

/// An operand that can be part of a list operand.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum JqlQueryUnitaryOperand {
    ValueOperand(ValueOperand),
    FunctionOperand(FunctionOperand),
    KeywordOperand(KeywordOperand),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::core::untagged::untagged!(JqlQueryUnitaryOperand { ValueOperand, FunctionOperand, KeywordOperand, Other });
