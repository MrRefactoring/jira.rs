// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of a parsed JQL query.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParsedJqlQuery {
    /// The list of syntax or validation errors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// The JQL query that was parsed and validated.
    pub query: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structure: Option<JqlQuery>,
    /// The list of warning messages
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}
