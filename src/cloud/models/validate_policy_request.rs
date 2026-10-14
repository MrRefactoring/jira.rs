// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ValidatePolicyRequest {
    #[serde(rename = "policyIds", default, skip_serializing_if = "Option::is_none")]
    pub policy_ids: Option<Vec<String>>,
}
