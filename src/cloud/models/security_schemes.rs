// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// List of security schemes.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecuritySchemes {
    /// List of security schemes.
    #[serde(rename = "issueSecuritySchemes", default, skip_serializing_if = "Option::is_none")]
    pub issue_security_schemes: Option<Vec<SecurityScheme>>,
}
