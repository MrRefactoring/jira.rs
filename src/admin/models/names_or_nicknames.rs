// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NamesOrNicknames {
    /// Names or nicknames filter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eq: Option<Vec<String>>,
    /// Partial name or nickname filter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,
}
