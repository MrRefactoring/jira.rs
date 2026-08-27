// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of a field configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldConfiguration {
    /// The description of the field configuration.
    pub description: String,
    /// The ID of the field configuration.
    pub id: i64,
    /// Whether the field configuration is the default.
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// The name of the field configuration.
    pub name: String,
}
