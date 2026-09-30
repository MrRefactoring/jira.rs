// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A result list containing objects and object type attributes
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectListInclTypeAttributesEntryResult {
    #[serde(rename = "startAt")]
    pub start_at: i64,
    #[serde(rename = "maxResults")]
    pub max_results: i64,
    /// The objects
    pub values: Vec<AssetObject>,
    /// The object type attributes
    #[serde(rename = "objectTypeAttributes")]
    pub object_type_attributes: Vec<ObjectTypeAttribute>,
    pub last: bool,
    #[serde(rename = "isLast")]
    pub is_last: bool,
}
