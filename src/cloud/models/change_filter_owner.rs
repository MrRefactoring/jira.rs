// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The account ID of the new owner.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangeFilterOwner {
    /// The account ID of the new owner.
    #[serde(rename = "accountId")]
    pub account_id: String,
}
