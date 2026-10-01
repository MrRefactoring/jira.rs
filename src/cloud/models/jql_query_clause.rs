// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::Serialize;

/// A JQL query clause.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum JqlQueryClause {
    CompoundClause(Box<CompoundClause>),
    FieldValueClause(FieldValueClause),
    FieldWasClause(FieldWasClause),
    FieldChangedClause(FieldChangedClause),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::core::untagged::untagged!(JqlQueryClause {
    CompoundClause,
    FieldValueClause,
    FieldWasClause,
    FieldChangedClause,
    Other
});
