// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The project issue type hierarchy.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Hierarchy {
    /// Details about the hierarchy level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub levels: Option<Vec<SimplifiedHierarchyLevel>>,
}
