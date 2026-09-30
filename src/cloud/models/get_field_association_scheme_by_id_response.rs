// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Response object for getting a field association scheme by ID.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetFieldAssociationSchemeByIdResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "fieldsCount", default, skip_serializing_if = "Option::is_none")]
    pub fields_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<FieldAssociationSchemeLinks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
