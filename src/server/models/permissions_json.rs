// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PermissionsJson {
    /// A map of permission keys to permission objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<std::collections::HashMap<String, serde_json::Value>>,
}
