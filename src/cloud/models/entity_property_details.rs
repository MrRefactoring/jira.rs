// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EntityPropertyDetails {
    /// The entity property ID.
    #[serde(rename = "entityId")]
    pub entity_id: f64,
    /// The entity property key.
    pub key: String,
    /// The new value of the entity property.
    pub value: String,
}
