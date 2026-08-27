// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The ID of a screen scheme.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDefaultScreenScheme {
    /// The ID of the screen scheme.
    #[serde(rename = "screenSchemeId")]
    pub screen_scheme_id: String,
}
