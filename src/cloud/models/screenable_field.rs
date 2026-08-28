// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A screen tab field.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ScreenableField {
    /// The ID of the screen tab field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the screen tab field. Required on create and update. The maximum length is 255 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
