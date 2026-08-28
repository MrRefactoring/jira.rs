// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UnrestrictedUserEmail {
    /// The accountId of the user
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The email of the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}
