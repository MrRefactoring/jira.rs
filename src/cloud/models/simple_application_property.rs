// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SimpleApplicationProperty {
    /// The ID of the application property.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The new value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
