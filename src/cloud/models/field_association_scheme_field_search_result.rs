// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Field association scheme field search results.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldAssociationSchemeFieldSearchResult {
    #[serde(rename = "allowedOperations", default, skip_serializing_if = "Option::is_none")]
    pub allowed_operations: Option<Vec<String>>,
    #[serde(rename = "fieldId", default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<SearchResultFieldParameters>,
    #[serde(rename = "restrictedToWorkTypes", default, skip_serializing_if = "Option::is_none")]
    pub restricted_to_work_types: Option<Vec<String>>,
    #[serde(rename = "workTypeParameters", default, skip_serializing_if = "Option::is_none")]
    pub work_type_parameters: Option<Vec<SearchResultWorkTypeParameters>>,
}
