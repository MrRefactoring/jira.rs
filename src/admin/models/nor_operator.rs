// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Returns workspaces excluding those that match any of the nested query variants. Absence of nested variants makes this operator no-op.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NorOperator {
    /// Returns workspaces excluding those that match any of the nested query variants. Absence of nested variants makes this operator no-op
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nor: Option<Vec<Box<QueryVariants>>>,
}
