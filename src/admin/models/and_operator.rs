// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Returns workspaces matching all of the nested query variants. Absence of nested variants makes this operator no-op.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AndOperator {
    /// Returns workspaces matching all of the nested query variants. Absence of nested variants makes this operator no-op.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<QueryVariants>>,
}
