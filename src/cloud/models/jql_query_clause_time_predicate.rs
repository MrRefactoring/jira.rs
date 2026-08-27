// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The operator between the field and the operand.
    pub enum JqlQueryClauseTimePredicateOperator {
        Before => "before",
        After => "after",
        From => "from",
        To => "to",
        On => "on",
        During => "during",
        By => "by",
    }
}

/// A time predicate for a temporal JQL clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JqlQueryClauseTimePredicate {
    pub operand: JqlQueryClauseOperand,
    /// The operator between the field and the operand.
    pub operator: JqlQueryClauseTimePredicateOperator,
}
