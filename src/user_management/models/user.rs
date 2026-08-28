// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub account_id: AccountId,
    pub account_type: AccountType,
    pub account_status: AccountStatus,
    pub name: Name,
    pub picture: Avatar,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub characteristics: Option<AccountCharacteristics>,
}
