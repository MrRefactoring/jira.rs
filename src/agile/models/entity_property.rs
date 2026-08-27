// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntityProperty {
    /// The key of the property.
    pub key: String,
    /// The value of the property.
    pub value: serde_json::Value,
}
