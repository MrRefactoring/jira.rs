// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RegisterItemHolder {
    #[serde(rename = "isLocked", default, skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(rename = "isManaged", default, skip_serializing_if = "Option::is_none")]
    pub is_managed: Option<bool>,
}
