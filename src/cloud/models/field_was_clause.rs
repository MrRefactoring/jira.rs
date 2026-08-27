// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The operator between the field and operand.
    pub enum FieldWasClauseOperator {
        Was => "was",
        WasIn => "was in",
        WasNotIn => "was not in",
        WasNot => "was not",
    }
}

/// A clause that asserts a previous value of a field. For example, `status WAS "Resolved" BY currentUser() BEFORE "2019/02/02"`. See [WAS](https://confluence.atlassian.com/x/dgiiLQ#Advancedsearching-operatorsreference-WASWAS) for more information about the WAS operator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldWasClause {
    pub field: JqlQueryField,
    pub operand: JqlQueryClauseOperand,
    /// The operator between the field and operand.
    pub operator: FieldWasClauseOperator,
    /// The list of time predicates.
    pub predicates: Vec<JqlQueryClauseTimePredicate>,
}
