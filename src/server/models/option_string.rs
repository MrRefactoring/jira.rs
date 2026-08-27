// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptionString {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defined: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    #[serde(rename = "orNull", default, skip_serializing_if = "Option::is_none")]
    pub or_null: Option<String>,
}
