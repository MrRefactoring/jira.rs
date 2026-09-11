// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserAnonymizationValidation {
    #[serde(rename = "affectedEntities", default, skip_serializing_if = "Option::is_none")]
    pub affected_entities: Option<std::collections::HashMap<String, Vec<AffectedEntity>>>,
    #[serde(rename = "businessLogicValidationFailed", default, skip_serializing_if = "Option::is_none")]
    pub business_logic_validation_failed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<std::collections::HashMap<String, ErrorCollection>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "userKey", default, skip_serializing_if = "Option::is_none")]
    pub user_key: Option<String>,
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warnings: Option<std::collections::HashMap<String, ErrorCollection>>,
}
