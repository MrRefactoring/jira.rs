// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IdOrKey {
    /// The ID of the referenced item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The key of the referenced item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
