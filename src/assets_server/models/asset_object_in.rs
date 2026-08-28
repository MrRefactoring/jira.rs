// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetObjectIn {
    #[serde(rename = "objectTypeId")]
    pub object_type_id: i64,
    pub attributes: Vec<ObjectAttributeIn>,
}
