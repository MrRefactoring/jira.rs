// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PolicyCreateInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<PolicyCreateModel>,
}
