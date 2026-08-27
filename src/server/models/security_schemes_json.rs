// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecuritySchemesJson {
    #[serde(rename = "issueSecuritySchemes", default, skip_serializing_if = "Option::is_none")]
    pub issue_security_schemes: Option<Vec<SecuritySchemeJson>>,
}
