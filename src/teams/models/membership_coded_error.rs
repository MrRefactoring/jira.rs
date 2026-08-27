// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MembershipCodedError {
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub code: String,
    pub message: String,
}
