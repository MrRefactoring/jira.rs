// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The details of the field configuration scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UpdateFieldConfigurationSchemeDetails {
    /// The description of the field configuration scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the field configuration scheme. The name must be unique.
    pub name: String,
}
