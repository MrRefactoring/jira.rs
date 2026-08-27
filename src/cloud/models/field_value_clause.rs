// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The operator between the field and operand.
    pub enum FieldValueClauseOperator {
        Value => "=",
        Value2 => "!=",
        Value3 => ">",
        Value4 => "<",
        Value5 => ">=",
        Value6 => "<=",
        In => "in",
        NotIn => "not in",
        Value7 => "~",
        Value8 => "~=",
        Is => "is",
        IsNot => "is not",
    }
}

/// A clause that asserts the current value of a field. For example, `summary ~ test`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValueClause {
    pub field: JqlQueryField,
    pub operand: JqlQueryClauseOperand,
    /// The operator between the field and operand.
    pub operator: FieldValueClauseOperator,
}
