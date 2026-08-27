// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormValidationErrorContext {
    /// ID of the form entity related to the validation error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Type of form entity related to the validation error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
