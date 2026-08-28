// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of a field configuration scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldConfigurationScheme {
    /// The description of the field configuration scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the field configuration scheme.
    pub id: String,
    /// The name of the field configuration scheme.
    pub name: String,
}
