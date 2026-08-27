// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddSecuritySchemeLevelsRequest {
    /// The list of scheme levels which should be added to the security scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub levels: Option<Vec<SecuritySchemeLevel>>,
}
