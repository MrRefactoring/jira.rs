// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateIssueSecuritySchemeRequest {
    /// The description of the security scheme scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the security scheme scheme. Must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
