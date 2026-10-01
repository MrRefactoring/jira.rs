// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::Serialize;

/// Possible operators/operand in the event query.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum QueryVariants {
    AndOperator(Box<AndOperator>),
    NorOperator(Box<NorOperator>),
    FieldOperand(FieldOperand),
    SearchWorkspacesOperand(SearchWorkspacesOperand),
    FeatureFilter(FeatureFilter),
    PolicyFilter(PolicyFilter),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::core::untagged::untagged!(QueryVariants {
    AndOperator,
    NorOperator,
    FieldOperand,
    SearchWorkspacesOperand,
    FeatureFilter,
    PolicyFilter,
    Other
});
