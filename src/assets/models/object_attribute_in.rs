// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Object attribute used for creating and updating
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ObjectAttributeIn {
    /// The type of the attribute. The type decides how this value should be interpreted
    #[serde(rename = "objectTypeAttributeId")]
    pub object_type_attribute_id: String,
    /// The value(s)
    #[serde(rename = "objectAttributeValues")]
    pub object_attribute_values: Vec<ObjectAttributeValueIn>,
}
