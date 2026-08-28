// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Identifies attributes to be displayed
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectTypeAttributesToDisplay {
    /// The identifier of the object type attributes to be displayed
    #[serde(rename = "attributesToDisplayIds")]
    pub attributes_to_display_ids: Vec<String>,
}
