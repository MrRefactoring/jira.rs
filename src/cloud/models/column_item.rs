// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of an issue navigator column item.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ColumnItem {
    /// The issue navigator column label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The issue navigator column value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
