// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssociateProjects {
    #[serde(rename = "idsOrKeys", default, skip_serializing_if = "Option::is_none")]
    pub ids_or_keys: Option<Vec<String>>,
}
