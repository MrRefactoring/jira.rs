// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// SCIM enterprise user extension
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnterpriseUserExtension {
    /// Organization the user belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Department the user belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
}
