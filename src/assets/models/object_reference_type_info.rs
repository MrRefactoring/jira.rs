// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Reference information for one object
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectReferenceTypeInfo {
    #[serde(rename = "referenceTypes", default, skip_serializing_if = "Option::is_none")]
    pub reference_types: Option<Vec<ReferenceType>>,
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<ObjectType>,
    #[serde(rename = "numberOfReferencedObjects")]
    pub number_of_referenced_objects: i64,
    #[serde(rename = "openIssuesExists")]
    pub open_issues_exists: bool,
}
