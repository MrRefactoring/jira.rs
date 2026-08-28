// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details about permissions.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Permissions {
    /// List of permissions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<std::collections::HashMap<String, serde_json::Value>>,
}
