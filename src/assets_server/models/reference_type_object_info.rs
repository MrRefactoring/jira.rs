// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReferenceTypeObjectInfo {
    #[serde(rename = "referenceTypes", default, skip_serializing_if = "Option::is_none")]
    pub reference_types: Option<Vec<ReferenceType>>,
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<ObjectType>,
    #[serde(rename = "numberOfReferencedObjects", default, skip_serializing_if = "Option::is_none")]
    pub number_of_referenced_objects: Option<i64>,
    #[serde(rename = "openIssuesExists", default, skip_serializing_if = "Option::is_none")]
    pub open_issues_exists: Option<bool>,
}
