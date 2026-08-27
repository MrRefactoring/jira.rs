// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Response object after successfully creating a new field association scheme.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateFieldAssociationSchemeResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<CreateFieldAssociationSchemeLinks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
