// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The operator between the clauses.
    pub enum CompoundClauseOperator {
        And => "and",
        Or => "or",
        Not => "not",
    }
}

/// A JQL query clause that consists of nested clauses. For example, `(labels in (urgent, blocker) OR lastCommentedBy = currentUser()). Note that, where nesting is not defined, the parser nests JQL clauses based on the operator precedence. For example, "A OR B AND C" is parsed as "(A OR B) AND C". See Setting the precedence of operators for more information about precedence in JQL queries.`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CompoundClause {
    /// The list of nested clauses.
    pub clauses: Vec<JqlQueryClause>,
    /// The operator between the clauses.
    pub operator: CompoundClauseOperator,
}
