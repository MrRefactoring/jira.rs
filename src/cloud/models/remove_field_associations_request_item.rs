// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Request item for removing field associations.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RemoveFieldAssociationsRequestItem {
    /// Set of scheme IDs from which to remove field associations
    #[serde(rename = "schemeIds")]
    pub scheme_ids: Vec<i64>,
}
