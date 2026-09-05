// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Duration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub millis: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub friendly: Option<String>,
}
