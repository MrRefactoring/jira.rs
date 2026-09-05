// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PolicyUpdateInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<PolicyUpdateModel>,
}
