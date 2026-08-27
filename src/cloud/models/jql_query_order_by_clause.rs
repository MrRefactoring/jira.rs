// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of the order-by JQL clause.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JqlQueryOrderByClause {
    /// The list of order-by clause fields and their ordering directives.
    pub fields: Vec<JqlQueryOrderByClauseElement>,
}
