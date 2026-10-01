// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::Serialize;

/// Details of an operand in a JQL clause.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum JqlQueryClauseOperand {
    ListOperand(ListOperand),
    ValueOperand(ValueOperand),
    FunctionOperand(FunctionOperand),
    KeywordOperand(KeywordOperand),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::core::untagged::untagged!(JqlQueryClauseOperand {
    ListOperand,
    ValueOperand,
    FunctionOperand,
    KeywordOperand,
    Other
});
