// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of a field configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldConfigurationDetails {
    /// The description of the field configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the field configuration. Must be unique.
    pub name: String,
}
