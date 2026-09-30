// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Request object for updating an existing field association scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UpdateFieldAssociationSchemeRequest {
    /// The description value to update
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name value to update
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
